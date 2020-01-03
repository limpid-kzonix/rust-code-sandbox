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
    println!("{}", measurement.ext_fmt());

    let addr: String = "127.0.0.1:8080".parse().unwrap_or("0.0.0.0:8080".to_owned());
    println!("{} ", addr.as_str());



    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((iron::status::Ok, "Hello World!")))
    }

    let _server = Iron::new(hello_world).http(addr.clone()).unwrap();
    println!("On {}", addr.to_owned());
}

struct Measurement {
    temperature: f32,
    scale: &'static str,
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
    fn ext_fmt(&self) -> String {
        return format!("{} {}", self.temperature.to_string(), &self.scale);
    }
}

trait MyTrait {
    fn ext_fmt(&self) -> String;
}

#[cfg(test)]
mod main {
    #[test]
    fn it_works() {
        println!("Boo!");
    }
}