extern crate futures;
extern crate tokio_core;
extern crate tiberius;

use futures::Future;
use tokio_core::reactor::Core;
use tiberius::SqlConnection;
use tiberius::stmt::ResultStreamExt;



fn main() {
    let mut lp = Core::new().unwrap();
    let connect_string = "server=tcp:127.0.0.1,1443;integratedSecurity=true;".to_owned();
    let query = "select Sum(ServiceCaseTime.TimeUsed) as Total, \
                 SUBSTRING(Customer.Name, 1,21) AS Kunde \
		         from [ServiceCaseTime] \
                 join [ServiceCase] on (ServiceCaseTime.ServiceCaseId = ServiceCase.Id) \
                 join [Customer] on (ServiceCase.CustomerId = Customer.Id) \
                 where [Done] > '2017-02-01' \
                 GROUP BY SUBSTRING(Customer.Name, 1,21) \
                 Order By Total DESC";
   let future = SqlConnection::connect(lp.handle(), connect_string.as_str()).and_then(|conn| {
       conn.simple_query(query).for_each_row(|row| {
           let val: i32 = row.get(0);
           assert_eq!(val, 3i32);
           Ok(())
       })
   });
   lp.run(future).unwrap();
}

