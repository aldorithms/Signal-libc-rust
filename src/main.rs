use libc::{sigaddset, sigemptyset, sigprocmask, SIGINT, SIG_BLOCK, SIG_UNBLOCK};
use std::thread;
use std::time::Duration;
fn main() 
{
    unsafe 
    {
        let mut mask: libc::sigset_t = std::mem::zeroed();
        sigemptyset(&mut mask);
        sigaddset(&mut mask, SIGINT);
        sigprocmask(SIG_BLOCK, &mask as *const libc::sigset_t, std::ptr::null_mut());
    }
    println!("Blocked SIGINT signal for 5 seconds");
    thread::sleep(Duration::from_secs(5));
    unsafe 
    {
        let mut mask: libc::sigset_t = std::mem::zeroed();
        sigemptyset(&mut mask);
        sigaddset(&mut mask, SIGINT);
        sigprocmask(SIG_UNBLOCK, &mask as *const libc::sigset_t, std::ptr::null_mut());
    }
    println!("Unblocked SIGINT signal");
}
