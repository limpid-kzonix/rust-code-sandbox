use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use iron::prelude::Iron;
use iron::Request;
use iron::IronResult;
use iron::Response;

fn main() {
    println!("Hello, world!");
    let measurement = Measurement { temperature: 36.6, scale: "Celsius" };
    println!("{}", measurement);

    let addr: String = "127.0.0.1:8080".parse().unwrap_or("0.0.0.0:8080".to_owned());
    println!("{} ", addr.as_str());

    measurement.ext_fmt();

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((iron::status::Ok, "Hello World!")))
    }

    let _server = Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}

struct Measurement {
    temperature: f32,
    scale: &'static str
}

impl Display for Measurement {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), Error> {
        let mut base_string = "Temperature is ".to_owned();
        //base_string.push_str(self.temperature.to_string().as_str());
        base_string.push_str(&(self.temperature.to_string().to_owned()));
        base_string.push_str(" by ");
        base_string.push_str(&(self.scale));
        _f.write_str(base_string.as_str())
    }
}

impl MyTrait for Measurement {
    fn ext_fmt(&self) {
        let string = format!("{} {}", self.temperature.to_string(), &self.scale);
        println!("{}", string)
    }
}

trait MyTrait {
    fn ext_fmt(&self);
}

#[cfg(test)]
mod main {
    #[test]
    fn it_works() {
        println!("Boo!");
    }
}