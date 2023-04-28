use super::messaging::{answer_generator::AnswerGenerator, pinger_job::PingerJob};

pub struct SocketManager<'a>{
    pub answer_generator: Option<&'a AnswerGenerator<'a>>,
    pub pinger_job: Option<&'a PingerJob<'a>>
}

impl<'a> SocketManager<'a> {
    pub fn set_answer_generator(&mut self, answer_generator: &'a AnswerGenerator) -> &mut SocketManager<'a> {
        self.answer_generator = Some(answer_generator);
        self
    }

    pub fn set_pinger_job(&mut self, pinger_job: &'a PingerJob) -> &mut SocketManager<'a> {
        self.pinger_job = Some(pinger_job);
        self
    }

}