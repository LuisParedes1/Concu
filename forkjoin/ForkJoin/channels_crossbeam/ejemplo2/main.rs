use crossbeam_channel::unbounded;

fn main() {
    let (s1, r1) = unbounded();
    let (s2, r2) = (s1.clone(), r1.clone());
    let (s3, r3) = (s2.clone(), r2.clone());
    
    s1.send(10).unwrap();
    s2.send(20).unwrap();
    s3.send(30).unwrap();
    
    assert_eq!(r3.recv(), Ok(10));
    assert_eq!(r1.recv(), Ok(20));
    assert_eq!(r2.recv(), Ok(30));
}
