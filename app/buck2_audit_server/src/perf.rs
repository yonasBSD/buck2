/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

//! Starlark debugging.

use async_trait::async_trait;
use buck2_audit::perf::AuditPerfCommand;
use buck2_cli_proto::ClientContext;
use buck2_server_ctx::ctx::ServerCommandContextTrait;
use buck2_server_ctx::partial_result_dispatcher::PartialResultDispatcher;

use crate::ServerAuditSubcommand;

#[async_trait]
impl ServerAuditSubcommand for AuditPerfCommand {
    async fn server_execute(
        &self,
        _server_ctx: &dyn ServerCommandContextTrait,
        _stdout: PartialResultDispatcher<buck2_cli_proto::StdoutBytes>,
        _client_ctx: ClientContext,
    ) -> buck2_error::Result<()> {
        Ok(())
    }
}
