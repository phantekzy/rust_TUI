use sysinfo::{Networks, System};

pub struct ProcessData {
    pub pid: String,
    pub name: String,
    pub cpu: f32,
    pub mem_mb: u64,
}

pub struct SysStats {
    pub cpu_usage: f32,
    pub mem_used: u64,
    pub mem_total: u64,
    pub swap_used: u64,
    pub swap_total: u64,
    pub net_in: u64,
    pub net_out: u64,
    pub load_avg: String,
    pub uptime: u64,
    pub processes: Vec<ProcessData>,
}

pub struct SystemTracker {
    sys: System,
    nets: Networks,
}

impl SystemTracker {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
            nets: Networks::new_with_refreshed_list(),
        }
    }

    pub fn get_stats(&mut self) -> SysStats {
        self.sys.refresh_all();
        self.nets.refresh(false);

        let cpu = self.sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>()
            / self.sys.cpus().len() as f32;

        let mut total_in = 0;
        let mut total_out = 0;
        for (_, data) in &self.nets {
            total_in += data.received();
            total_out += data.transmitted();
        }

        let mut procs: Vec<_> = self.sys.processes().values().collect();
        procs.sort_by(|a, b| {
            b.cpu_usage()
                .partial_cmp(&a.cpu_usage())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let top_procs = procs
            .iter()
            .take(50)
            .map(|p| ProcessData {
                pid: p.pid().to_string(),
                name: p.name().to_string_lossy().into_owned(),
                cpu: p.cpu_usage(),
                mem_mb: p.memory() / 1024 / 1024,
            })
            .collect();

        let load = System::load_average();

        SysStats {
            cpu_usage: cpu,
            mem_used: self.sys.used_memory(),
            mem_total: self.sys.total_memory(),
            swap_used: self.sys.used_swap(),
            swap_total: self.sys.total_swap(),
            net_in: total_in,
            net_out: total_out,
            load_avg: format!("{:.2}, {:.2}, {:.2}", load.one, load.five, load.fifteen),
            uptime: System::uptime(),
            processes: top_procs,
        }
    }
}
