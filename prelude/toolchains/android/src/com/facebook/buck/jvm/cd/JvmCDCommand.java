/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

package com.facebook.buck.jvm.cd;

import java.io.IOException;

/** A single compilation action created from command line or worker args */
public interface JvmCDCommand {
  String WORKING_DIRECTORY_ENV_VAR = "BUCK_SCRATCH_PATH";

  BuildCommandStepsBuilder getBuildCommand();

  String getActionId();

  int getLoggingLevel();

  void postExecute() throws IOException;
}
