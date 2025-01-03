/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use buck2_cli_proto::SetLogFilterRequest;
use buck2_client_ctx::client_ctx::ClientCommandContext;
use buck2_client_ctx::common::BuckArgMatches;
use buck2_client_ctx::daemon::client::connect::BuckdConnectOptions;
use buck2_client_ctx::exit_result::ExitResult;

/// Change the log filter that's currently applied by the Buck2 daemon.
#[derive(Debug, clap::Parser)]
#[clap()]
pub struct SetLogFilterCommand {
    /// The log filter to apply.
    #[clap()]
    log_filter: String,

    /// Whether not to apply it to the daemon.
    #[clap(long)]
    no_daemon: bool,

    /// Whether not to apply it to the forkserver.
    #[clap(long)]
    no_forkserver: bool,
}

impl SetLogFilterCommand {
    pub fn exec(self, _matches: BuckArgMatches<'_>, ctx: ClientCommandContext<'_>) -> ExitResult {
        ctx.with_runtime(|ctx| async move {
            let mut buckd = ctx
                .connect_buckd(BuckdConnectOptions::existing_only_no_console())
                .await?;

            buckd
                .with_flushing()
                .set_log_filter(SetLogFilterRequest {
                    log_filter: self.log_filter,
                    daemon: !self.no_daemon,
                    forkserver: !self.no_forkserver,
                })
                .await?;

            ExitResult::success()
        })
    }
}
