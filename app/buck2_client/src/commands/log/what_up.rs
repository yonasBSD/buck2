/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use std::time::SystemTimeError;

use buck2_client_ctx::client_ctx::BuckSubcommand;
use buck2_client_ctx::client_ctx::ClientCommandContext;
use buck2_client_ctx::common::BuckArgMatches;
use buck2_client_ctx::events_ctx::EventsCtx;
use buck2_client_ctx::exit_result::ExitResult;
use buck2_client_ctx::subscribers::superconsole::CUTOFFS;
use buck2_client_ctx::subscribers::superconsole::StatefulSuperConsole;
use buck2_client_ctx::subscribers::superconsole::SuperConsoleConfig;
use buck2_client_ctx::subscribers::superconsole::SuperConsoleState;
use buck2_client_ctx::subscribers::superconsole::session_info::SessionInfoComponent;
use buck2_client_ctx::subscribers::superconsole::timed_list::TimedList;
use buck2_error::conversion::from_any_with_tag;
use buck2_event_log::stream_value::StreamValue;
use buck2_event_observer::verbosity::Verbosity;
use buck2_events::BuckEvent;
use superconsole::Component;
use superconsole::Dimensions;
use superconsole::DrawMode;
use superconsole::Lines;
use superconsole::components::DrawVertical;
use tokio_stream::StreamExt;

use crate::commands::log::options::EventLogOptions;

/// Show the spans that were open when the log ended.
#[derive(Debug, clap::Parser)]
pub struct WhatUpCommand {
    #[clap(flatten)]
    event_log: EventLogOptions,

    /// Show spans after X amount of milliseconds
    #[clap(
        long,
        help = "Print the actions that where open after certain amount of milliseconds",
        value_name = "NUMBER"
    )]
    pub after: Option<u64>,
}

impl BuckSubcommand for WhatUpCommand {
    const COMMAND_NAME: &'static str = "log-what-up";

    async fn exec_impl(
        self,
        _matches: BuckArgMatches<'_>,
        ctx: ClientCommandContext<'_>,
        _events_ctx: &mut EventsCtx,
    ) -> ExitResult {
        let Self { event_log, after } = self;
        let cutoff_time = after.map(Duration::from_millis);

        let log_path = event_log.get(&ctx).await?;

        // Get events
        let (invocation, mut events) = log_path.unpack_stream().await?;

        let mut super_console = StatefulSuperConsole::console_builder()
            .build_forced(StatefulSuperConsole::FALLBACK_SIZE)
            .map_err(|e| from_any_with_tag(e, buck2_error::ErrorTag::LogCmd))?;

        let mut super_console_state = SuperConsoleState::new(
            None,
            invocation.trace_id,
            Verbosity::default(),
            true,
            SuperConsoleConfig {
                max_lines: 1000000,
                ..Default::default()
            },
            None,
        )?;
        let mut first_timestamp = None;
        // Ignore any events that are truncated, hence unreadable
        while let Ok(Some(event)) = events.try_next().await {
            match event {
                StreamValue::Event(event) => {
                    let e = BuckEvent::try_from(event)?;
                    match cutoff_time {
                        Some(cutoff_time) => {
                            if should_stop_reading(
                                cutoff_time,
                                e.timestamp(),
                                *first_timestamp.get_or_insert(e.timestamp()),
                            )? {
                                break;
                            }
                        }
                        _ => (),
                    }

                    super_console_state
                        .update_event_observer(&Arc::new(e))
                        .await?;
                }
                StreamValue::PartialResult(..) => {}
                StreamValue::Result(result) => {
                    let result = StatefulSuperConsole::render_result_errors(&result);
                    super_console.emit(result);
                    super_console
                        .finalize(&Self::component(&super_console_state))
                        .map_err(|e| from_any_with_tag(e, buck2_error::ErrorTag::LogCmd))?;
                    buck2_client_ctx::eprintln!("No open spans to render when log ended")?;
                    return ExitResult::success();
                }
            }
        }

        super_console
            .finalize_with_mode(&Self::component(&super_console_state), DrawMode::Normal)
            .map_err(|e| from_any_with_tag(e, buck2_error::ErrorTag::LogCmd))?;
        ExitResult::success()
    }
}

impl WhatUpCommand {
    fn component(state: &SuperConsoleState) -> impl Component + '_ {
        struct ComponentImpl<'a> {
            state: &'a SuperConsoleState,
        }

        impl Component for ComponentImpl<'_> {
            fn draw_unchecked(
                &self,
                dimensions: Dimensions,
                mode: DrawMode,
            ) -> anyhow::Result<Lines> {
                let mut draw = DrawVertical::new(dimensions);
                draw.draw(
                    &SessionInfoComponent {
                        session_info: self.state.session_info(),
                    },
                    mode,
                )?;
                draw.draw(&TimedList::new(&CUTOFFS, self.state), mode)?;
                Ok(draw.finish())
            }
        }

        ComponentImpl { state }
    }
}

fn should_stop_reading(
    after: Duration,
    event: SystemTime,
    first: SystemTime,
) -> Result<bool, SystemTimeError> {
    let elapsed = event.duration_since(first)?;
    if elapsed > after {
        return Ok(true);
    }
    Ok(false)
}
