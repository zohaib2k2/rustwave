use tokio::task;

#[derive(Debug)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug)]
pub struct Wave {
    pub domain_subset: Vec<String>,
    pub subsection: Vec<String>,
    pub subdom_range: Range,
}

impl Wave {
    pub fn new(domains_list: Vec<String>, subsection: Vec<String>, subdomain_range: Range) -> Self {
        Wave {
            domain_subset: domains_list,
            subsection: subsection,
            subdom_range: subdomain_range,
        }
    }

    pub async fn execute(&self, nthreads: u8) {
        for domain in self.domain_subset.iter() {
            for sub_windows in self.subsection.chunks(nthreads.into()) {
                let mut tasks = vec![];

                for window_idx in sub_windows {
                    let domain = domain.clone();
                    let sub = window_idx.clone();

                    // create async task and collect.
                    let task = task::spawn(async move {
                        Wave::check_domain(&sub, &domain).await;
                    });
                    tasks.push(task)
                }

                // complete this task and don't continue the loop until completed
                let results = futures::future::join_all(tasks).await;
                // Handle errors if any tasks failed
                for result in results {
                    if let Err(e) = result {
                        eprintln!("Task failed: {:?}", e);
                    }
                }
            }
        }
    }

    pub async fn check_domain(sub: &str, domain: &str) {
        let status_code = reqwest::get(format!("https://{}.{}", sub, domain)).await;

        if status_code.is_ok() {
            let code = status_code.unwrap().status();

            if code == 200 || code == 302 || code == 301 || code == 307 || code == 308 {
                println!("{}.{}", sub, domain);
            }
        }

        //tokio::time::sleep(std::time::Duration::from_millis(1)).await; // Simulated async work
    }
}
