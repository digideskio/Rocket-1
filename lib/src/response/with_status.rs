use response::{Responder, ResponseOutcome};
use http::hyper::{StatusCode, FreshHyperResponse};

pub struct StatusResponse<R: Responder> {
    status: StatusCode,
    responder: R,
}

impl<R: Responder> StatusResponse<R> {
    pub fn new(status: StatusCode, responder: R) -> StatusResponse<R> {
        StatusResponse {
            status: status,
            responder: responder,
        }
    }
}

impl<R: Responder> Responder for StatusResponse<R> {
    fn respond<'b>(&mut self, mut res: FreshHyperResponse<'b>) -> ResponseOutcome<'b> {
        *(res.status_mut()) = self.status;
        self.responder.respond(res)
    }
}
