use std::thread::{self, Thread};
use std::sync::{mpsc,Arc,Mutex};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct Worker{
    id : usize,
    thread: thread::JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

trait FnBox{
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce() + ?Sized> FnBox for F {
    fn call_box(self: Box<F>) {
        (self)();
    }
}

impl ThreadPool {
    /// 새 ThreadPool 인스턴스를 생성한다.
    /// 
    /// size 매개변수의 풀의 스레드 개수를 지정한다.
    ///
    /// # Panics
    /// 
    /// size 매개변수의 값이 '0'이면 new 함수는 assert 발생
    pub fn new(size: usize)->ThreadPool{
        assert!(size>0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0.. size{
            //thread 생성해서 vector에 저장
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool{
            workers,
            sender
        }
    }

    pub fn excute<F>(&self, f:F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

impl Worker {
    pub fn new (id : usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>)->Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("시작: 작업자 {}",id);

                job.call_box();
            }
        });

        Worker { id: id, thread: thread }
    }
}