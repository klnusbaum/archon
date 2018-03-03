use messaging;
use messaging::{Message, RaftMessage};
use types::{PeerId, Term};
use persistence;

enum State {
    Follower,
    Candidate,
    Leader,
}

pub struct Algorithm<TX, SS>
where
    TX: messaging::MessageSender,
    SS: persistence::StateStore,
{
    message_sender: TX,
    state_store: SS,
    my_id: PeerId,
    peers: Vec<PeerId>,
}

impl<TX, SS> Algorithm<TX, SS>
where
    TX: messaging::MessageSender,
    SS: persistence::StateStore,
{
    pub fn new(
        message_sender: TX,
        state_store: SS,
        peers: Vec<PeerId>,
        my_id: PeerId,
    ) -> Algorithm<TX, SS> {
        Algorithm {
            message_sender,
            state_store,
            peers,
            my_id,
        }
    }

    pub fn start<RX: messaging::MessageStream>(&mut self, message_stream: &mut RX) {
        for message in message_stream {
            match message {
                Message::RaftMessage(raft_message) => self.handle_raft_message(raft_message),
                Message::ElectionTimeout => self.handle_election_timeout(),
            };
        }
    }

    fn handle_raft_message(&mut self, raft_message: RaftMessage) {
        match raft_message {
            RaftMessage::AppendRequest(body) => self.handle_append_request(body),
            RaftMessage::AppendResponse(body) => self.handle_append_response(body),
            RaftMessage::VoteRequest(body) => self.handle_vote_request(body),
            RaftMessage::VoteResponse(body) => self.handle_vote_response(body),
        };
    }

    fn handle_append_request(&mut self, body: messaging::AppendRequestBody) {
        let current_term = self.state_store.get_current_term();
        if body.term < current_term {
            let response_body = messaging::AppendResponseBody {
                to_peer: body.from_peer,
                term: current_term,
                result: messaging::AppendResult::Failed,
            };
            self.message_sender
                .send_message(RaftMessage::AppendResponse(response_body));
        }
    }

    fn handle_append_response(&mut self, body: messaging::AppendResponseBody) {}

    fn handle_vote_request(&mut self, body: messaging::VoteRequestBody) {}
    fn handle_vote_response(&mut self, body: messaging::VoteResponseBody) {}
    fn handle_election_timeout(&mut self) {}
}
