mod russoerror;
mod server;

use russoerror::SSOError;

trait SSOConnector<A> {
    type R: Clone;

    fn authenticate(&mut self) -> Result<A, SSOError>;

    fn roles(&mut self) -> Result<Vec<Self::R>, SSOError>;
}
