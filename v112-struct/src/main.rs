struct Persion {
    name: String,
    job: Option<Job>,
}

struct Job {
    position: String,
    phone: Option<Phone>,
}

struct Phone {
    default: bool,
    number: Option<u32>,
}

impl Persion {
    fn persion_name(&self) -> String {
        self.name.clone()
    }
    fn job_position(&self) -> String {
        match &self.job {
            Some(job) => job.position.clone(),
            None => String::from(""),
        }
    }
    fn phone(&self) -> u32 {
        self.job
            .as_ref()
            .unwrap()
            .phone
            .as_ref()
            .unwrap()
            .number
            .unwrap()
    }
    fn job(&self) -> String {
        self.job
            .as_ref()
            .unwrap()
            .phone
            .as_ref()
            .unwrap()
            .number
            .unwrap()
            .to_string()
    }
}

fn main() {
    let p = Persion {
        job: Some(Job {
            phone: Some(Phone {
                number: Some(123456789),
                default: true,
            }),
            position: "Software Engineer".to_string(),
        }),
        name: String::from("John"),
    };

    println!("{}", p.phone());
    println!("{}", p.job());
    println!("{}", p.job_position());
    println!("{}", p.persion_name());
}
