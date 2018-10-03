extern crate iron;
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;
use std::str::FromStr;

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

    let form_data = match request.get_ref::<UrlEncodedBody>(){
        Err(e) => {
            resp.set_mut(status::BadRequest);
            resp.set_mut(format!("error parsing data {:?}\n", e));
            return Ok(resp)
        }
        Ok(map) => map
    };
    let unparsed_numbers = match form_data.get("n") {
        None => {
            resp.set_mut(status::BadRequest);
            resp.set_mut(format!("form data has no 'n' paramter\n"));
            return Ok(resp)
        }
        Some(nums) => nums
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                resp.set_mut(status::BadRequest);
                resp.set_mut(format!("Value for 'n' is not a number {:?}\n", unparsed));
                return Ok(resp)
            }
            Ok(n) => { numbers.push(n); }
        }
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    resp.set_mut(status::Ok);
    resp.set_mut(mime!(Text/Html;Charset=Utf8));
    resp.set_mut(format!("The GCD of {:?} is <b>{}</b>\n", numbers, d));
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