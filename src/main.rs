extern crate iron;
extern crate router;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main(){
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "cgd");

    println!("Serving on localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
 let mut resp = Response::new();
    resp.set_mut(status::Ok);
    resp.set_mut(mime!(Text/Html; Charset=Utf8));
    resp.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
          <input type="text" name="n"/>
          <input type="text" name="n"/>
          <button type="submit">Compute GCD</button>
        </form>
        "#);
    Ok(resp)
}

fn post_gcd(request: &mut Request) -> IronResult<Response>{
    let mut resp = Response::new();

    resp.set_mut(status::Ok);
    resp.set_mut(r#"OK!"#);

    Ok(resp)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}