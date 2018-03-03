use types::{LogEntry, LogIndex, PeerId, Term};

pub enum Message {
    RaftMessage(RaftMessage),
    ElectionTimeout,
}

pub enum RaftMessage {
    AppendRequest(AppendRequestBody),
    AppendResponse(AppendResponseBody),
    VoteRequest(VoteRequestBody),
    VoteResponse(VoteResponseBody),
}

pub trait MessageStream: Iterator<Item = Message> {
    fn reset_election_timeout(&mut self);
}

pub trait MessageSender {
    fn send_message(&mut self, raft_message: RaftMessage);
}

pub struct AppendRequestBody {
    pub from_peer: PeerId,
    pub term: Term,
    pub leader_id: PeerId,
    pub prev_log_index: LogIndex,
    pub prev_log_term: Term,
    pub entries: Vec<LogEntry>,
    pub leader_commit: LogIndex,
}

pub struct AppendResponseBody {
    pub to_peer: PeerId,
    pub term: Term,
    pub result: AppendResult,
}

pub enum AppendResult {
    Successful,
    Failed,
}

pub struct VoteRequestBody {
    pub from_peer: PeerId,
    pub term: Term,
    pub candidate_id: PeerId,
    pub last_log_index: LogIndex,
    pub last_log_term: Term,
}

pub struct VoteResponseBody {
    pub to_peer: PeerId,
    pub term: Term,
    pub vote_granted: Vote,
}

pub enum Vote {
    Grant,
    Deny,
}
