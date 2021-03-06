use crate::cli::{CliHandler, CliMiddleware};
use clap::{App, ArgMatches, Arg};
use std::error::Error;
use crate::tx::{TxBuilder, StdSignature};
use crate::result::Res;
use regen_client_sdk::auth::PubKey;
use regen_context::SimpleContext;
use crate::error::ABCIError;

pub trait KeyBase {
    fn sign(&self, key: &str, bytes: &[u8], str_rep: &str) -> Res<SignRes>;
}

pub struct SignRes {
    pub sig: Box<[u8]>,
    pub pub_key: Box<dyn PubKey>
}

struct SigCli {
    key_base: Box<dyn KeyBase>
}

const FROM: &'static str = "from";

impl CliMiddleware<dyn TxBuilder> for SigCli {
    fn on_build_cli_app(&self, ctx: &SimpleContext, app: App, next: &dyn CliHandler<&dyn TxBuilder>) -> App {
        next.build_cli_app(
            ctx,
            app.arg(Arg::with_name(FROM)
                .long(FROM)
                .value_name("KEY")
                .value_delimiter(",")
                .help("signs the transaction using the named key(s)")
            )
        )
    }

    fn on_run_cli_app(&self, ctx: &SimpleContext, matches: ArgMatches, next: &dyn CliHandler<&dyn TxBuilder>) -> Res<&dyn TxBuilder> {
        let mut bldr = next.run_cli_app(ctx, matches)?;
        let keys = matches.values_of(FROM).ok_or(ABCIError::NotFound)?;
        for key in keys.iter() {
            let sign_res = self.key_base.sign(key, bldr.get_sign_bytes(), bldr.get_msg_text())?;
            // TODO: get sequence
            // bldr = bldr.with_signature(sign_res.pub_key, sign_res.sig, )
        }
        bldr
    }
}