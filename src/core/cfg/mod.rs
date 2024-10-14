/*
Copyright 2024 The openBCE Authors.
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
    http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    /// Run a container directly
    Runc {
        /// The yaml file of container to run
        #[arg(short, long)]
        file: String,
    },

    /// Run a Pod directly
    Runp,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct ChariotOptions {
    #[command(subcommand)]
    pub command: Commands,
}