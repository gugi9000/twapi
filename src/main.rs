extern crate futures;
extern crate tokio_core;
extern crate tiberius;

use futures::Future;
use tokio_core::reactor::Core;
use tiberius::SqlConnection;
use tiberius::stmt::ResultStreamExt;
use std::env;



fn main() {
    let mut lp = Core::new().unwrap();
    let sqlserver = &env::var("TW_SQL").expect("Set TW_QPI to something like \"localhost,1443\"");
    let connect_string = format!("server=tcp:{};integratedSecurity=true;Database=RIS;", sqlserver).to_owned();
    let query = "select 
		Sum(ServiceCaseTime.TimeUsed) as Total
      , SUBSTRING(Customer.Name, 1,21) AS Kunde
		from [ServiceCaseTime]
    		join [ServiceCase] on (ServiceCaseTime.ServiceCaseId = ServiceCase.Id)
		    join [Customer] on (ServiceCase.CustomerId = Customer.Id)
		where [Done] > '2017-02-01'
		GROUP BY SUBSTRING(Customer.Name, 1,21)
		Order By Total DESC";

   let future = SqlConnection::connect(lp.handle(), connect_string.as_str()).and_then(|conn| {
       conn.simple_query(query).for_each_row(|row| {
           let kunde: &str = row.get("Kunde");
           let timer: f64 = row.get("Total");
           println!("\t{{\"{}\": {}}},", kunde, timer);
           Ok(())
       })
   }); 

   println!("{{\"kundetimer\": [");
   lp.run(future).unwrap();
   println!("]}}");

}

fn getHoursByMonth(year: i32, month: i32) -> i32 {
    //Returnerer antal timer i angivet måned og år
    // Sidste måned:             getHrs(date('Y-m', mt(-1, 0)), date('Y-m'))
    // Denne måned sidste år:    getHrs(date('Y-m', mt(0, -1)), date('Y-m', mt(1, -1)))
    
    // $append = ($until) ? "AND [Done] < '{$until}-01 0:00:00.000'" : '' ;
    // $query = mssql_query("SELECT [TimeUsed]
    //     FROM [RIS].[dbo].[ServiceCaseTime] WHERE  '{$aar_maaned}-01 0:00:00.000' < [Done] {$append}
    //     ORDER BY [Done] DESC
    // ");
    let mut hours = 0;
    // $timer = 0;
    // do {
    //     while ($row = mssql_fetch_row($query)) {
    //         $timer = $timer + $row[0];
    //     }
    // } while (mssql_next_result($query));
    // mssql_free_result($query);

    hours
}
fn getCustomerByMonth(year: i32, month: i32) -> i32 {
    let mut customers = 0;

    // $start = $year."-".$month."-01";
    // if ($month == "12") { 
    //         $year++; 
    //         $month='00';
    // }
    // $end = $year."-".($month+1)."-01";
    let start = format!("{}-{}-01",year.to_string(), month.to_string());
    let end = format!("{}-{}-01",year.to_string(), month.to_string());
    
    let query = format!("select 
                Sum(ServiceCaseTime.TimeUsed) as Total,
                SUBSTRING(Customer.Name, 1,21) AS Kunde
                from [ServiceCaseTime]
                join [ServiceCase] on (ServiceCaseTime.ServiceCaseId = ServiceCase.Id)
                join [Customer] on (ServiceCase.CustomerId = Customer.Id)
                where [Done] > '{}' AND [Done] < '{}'
                GROUP BY SUBSTRING(Customer.Name, 1,21)
                Order By Total DESC", start, end);
    // $query = mssql_query($stmt);
    // $rows = 0;
    // do {
    //         while ($row = mssql_fetch_row($query)) {
    //                 $rows ++;
    //         }
    // } while (mssql_next_result($query));
    // mssql_free_result($query);
    customers 

}

fn getHoursLastXDays (days: i32) -> Vec<(i32, i32)> {
    // return vec of hours registered each day last X days
    // $stamp = date("Y-m-d", mktime(0, 0, 0, date('m'), date('j')-29, date('Y')));

    // $query = mssql_query("SELECT [TimeUsed], [Done]
    //     FROM [RIS].[dbo].[ServiceCaseTime] WHERE [InvoiceType] != 3 AND '{$stamp} 0:00:00.000' < [Done]
    //     ORDER BY [Done] DESC
    // ");

    // $array_of_days = array_fill(0, 30, 0);

    // do {
    //     while ($row = mssql_fetch_row($query)) {
    //     $then = new DateTime(date('Y-m-d', mktime(0, 0, 0, date('m'), date('j')-29., date('Y'))));
    //     $now  = new DateTime($row[1]);

    //     $i = $then->diff($now)->format("%a");

    //     $array_of_days[$i] += $row[0];
    //     }
    // } while (mssql_next_result($query));
    // mssql_free_result($query);

    // return $array_of_days;
    vec![(-1, 23),(0, 3)]
}


fn getNumberOfWorkingDays() {
    // $from, $to) {
    // $workingDays = [1, 2, 3, 4, 5]; # date format = N (1 = Monday, ...)
    // $holidayDays = ['*-01-01', '*-12-24', '*-12-25', '*-12-26', 
    //                 '2017-04-13', '2017-04-14', '2017-04-17', '2017-05-12', 
    //                 '2017-05-25', '2017-06-05']; # variable and fixed holidays

    //     // År 	Palmesøndag 	Skærtorsdag 	Langfredag 	Påskedag 	2. Påskedag 	St. Bededag 	Kr. H.fartsdag 	Pinsedag 	2. Pinsedag
    //     // 2017 	9. april 	13. april 	14. april 	16. april 	17. april 	12. maj 	25. maj 	4. juni 	5. juni
    //     // 2018 	25. marts 	29. marts 	30. marts 	1. april 	2. april 	27. april 	10. maj 	20. maj 	21. maj
    //     // 2019 	14. april 	18. april 	19. april 	21. april 	22. april 	17. maj 	30. maj 	9. juni 	10. juni
    //     // 2020 	5. april 	9. april 	10. april 	12. april 	13. april 	8. maj 	21. maj 	31. maj 	1. juni
    //     // 2021 	28. marts 	1. april 	2. april 	4. april 	5. april 	30. april 	13. maj 	23. maj 	24. maj
    //     // 2022 	10. april 	14. april 	15. april 	17. april 	18. april 	13. maj 	26. maj 	5. juni 	6. juni
    //     // 2023 	2. april 	6. april 	7. april 	9. april 	10. april 	5. maj 	18. maj 	28. maj 	29. maj
    //     // 2024 	24. marts 	28. marts 	29. marts 	31. marts 	1. april 	26. april 	9. maj 	19. maj 	20. maj
    //     // 2025 	13. april 	17. april 	18. april 	20. april 	21. april 	16. maj 	29. maj 	8. juni 	9. juni
    //     // 2026 	29. marts 	2. april 	3. april 	5. april 	6. april 	1. maj 	14. maj 	24. maj 	25. maj
    //     // 2027 	21. marts 	25. marts 	26. marts 	28. marts 	29. marts 	23. april 	6. maj 	16. maj 	17. maj
    //     // 2028 	9. april 	13. april 	14. april 	16. april 	17. april 	12. maj 	25. maj 	4. juni 	5. juni
    //     // 2029 	25. marts 	29. marts 	30. marts 	1. april 	2. april 	27. april 	10. maj 	20. maj 	21. maj
    //     // 2030 	14. april 	18. april 	19. april 	21. april 	22. april 	17. maj 	30. maj 	9. juni 	10. juni                    

    // $from = new DateTime($from);
    // $to = new DateTime($to);
    // $to->modify('+1 day');
    // $interval = new DateInterval('P1D');
    // $periods = new DatePeriod($from, $interval, $to);

    // $days = 0;
    // foreach ($periods as $period) {
    //     if (!in_array($period->format('N'), $workingDays)) continue;
    //     if (in_array($period->format('Y-m-d'), $holidayDays)) continue;
    //     if (in_array($period->format('*-m-d'), $holidayDays)) continue;
    //     $days++;
    // }
    // return $days;
}

fn printStats() {
    // $current_month = getHrs(date('Y-m'));
    // print "<p>Current month: ". $current_month . "</p>";
    // $last_month = getHrs(date('Y-m', mt(-1, 0)), date('Y-m'));
    // print "<p>Last month: ". $last_month . "</p>";
    // print "<p>This month last year: ". getHrs(date('Y-m', mt(0, -1)), date('Y-m', mt(1, -1))). "</p>";

    // $this_year = getHrs(date('Y-m', mt(1, 0, true)));
    // print "<p>This year: ". $this_year . "</p>";
    // $year_start = new DateTime(date('Y')."-01-01");
    // $today = new DateTime(date('Y-m-d'));
    // $diff = $today->diff($year_start)->format("%a");
    // $workdaysThisYear = number_of_working_days(date(date('Y')."-01-01"), date(date('Y')."-12-31"));
    // //print $workdaysThisYear;
    // //$divisor = 12 * $diff/365; //Dette er ikke helt præcist, da den ikke tager hensyn til skudår og lign.
    // $divisor = 12 * $diff/$workdaysThisYear; // in 2017 that's 252 rather than 365
    // //print "<p>Monthly average so far: ". round($this_year/$divisor, 2) . "</p>"; // I have no idea what this means
    // print "<p>Last year: ". getHrs(date('Y-m', mt(1, -1, true)), date('Y-m', mt(1, 0, true))). "</p>";
    // print "<p>Monthly avg. last year: ". (round(getHrs(date('Y-m', mt(1, -1, true)), date('Y-m', mt(1, 0, true)))/12,2)) . "</p>";


    // // $days_in_current_month = date("t"); // included weekend and holidays
    // $days_in_current_month = number_of_working_days(date('Y-m-d', mt(0, 0)), date('Y-m-d',  mt(+1, 0)-1));
    // $day_of_month = date("d"); // Needs to be number og work days up to today
    // $day_of_month = number_of_working_days(date('Y-m-d', mt(0, 0)), date("Y-m-d"));
    // $projected_for_this_month = ( (getHrs(date('Y-m')) / $day_of_month) * $days_in_current_month );
    // print "<p>Projected this month: ". round($projected_for_this_month, 2) . "</p>";

    // $past_months = getHrs(date('Y').'-01', date('Y-m', mt(0, 0)));
    // print "<p>Projected monthly avg. for this year: ". round( ($past_months + $projected_for_this_month)/date("m") , 2) . "</p>";
    // //print "<p>Projected work days this month: ". $days_in_current_month . "</p>";
}


fn getPendingCases() {
    // get unmanged service cases    
    // $query = mssql_query('
    // SELECT [Received]
    //     ,[FromName]
    //     ,[FromEmail]
    //     ,[Subject]
    //     ,[TimeoutNotificationSent]
    //     ,[MailCaseMailboxId]
    // FROM [RIS].[dbo].[MailCase] 
    
    // where
    // [HandledByEmployeeId] IS NULL 
    // -- and
    // -- [MailCaseMailboxId] = 10011
    // ORDER BY [Received] DESC
    // ');
    // $json = '{';
    // $json .= ' "cases": [';
    // do {
    //     while ($row = mssql_fetch_row($query)) {
    //         $json .= '{';
    //         $json .= "\"modtaget\": \"".date("Y-m-d H:i", strtotime($row[0]))."\",\n";
    //         $json .= "\"afsender\": \"".$row[1]."\",\n";
    //         $json .= " \"email\": \"".$row[2]."\",\n";
    //         $json .= "\"emne\": \"".htmlentities($row[3])."\",\n";
    //         $json .= "\"timedout\": \"".$row[4]."\",\n";
    //         $json .= "\"mailbox\": \"".$row[5]."\"\n";
    //         $json .= "},\n";
    //         }
    //     } while (mssql_next_result($query));
    // mssql_free_result($query);
    // $json = substr($json, 0, -2);
    // $json .= "]";
    // $json .= "}";
    //    if (count($json)<14) { $json = "{\"cases\":[] }"; }
    // return $json;
}

fn getManagedServiceCases(amount: i32) -> String {
    // get latest X managed service cases
    // $query2 = mssql_query('
    // SELECT TOP '.$n.'
    //     [Received]
    //     ,[FromName]
    //     ,[FromEmail]
    //     ,[Subject]
    //     ,[Name]
    //     ,[HandledAt]
    //     ,[mailcasemailboxid]
    // FROM [RIS].[dbo].[MailCase] JOIN [RIS].[dbo].[Employee] ON (Mailcase.HandledByEmployeeId = Employee.Id) where
    // -- mailcasemailboxid = 10011 and
    //  handledat != \'\' order by HandledAt desc 
    // ');

    // $json = "{ \"cases\":\n";
    // $json .= "    [\n";
    // do {
    //     while ($row = mssql_fetch_row($query2)) {
    //     $json .= "    {\n";
    //     $json .= "      \"received\": \"".date("Y-m-d H:i", strtotime($row[0]))."\",\n";
    //     $json .= "      \"sender\": \"".$row[1]."\",\n";
    //     $json .= "      \"email\": \"".$row[2]."\",\n";
    //     $json .= "      \"subject\": \"".urlencode($row[3])."\",\n";
    //     $json .= "      \"name\": \"".$row[4]."\",\n";
    //     $json .= "      \"mailbox\": \"".$row[6]."\",\n";
    //     $json .= "      \"handled\": \"" .date("Y-m-d H:i", strtotime($row[5]))."\"\n";
    //     $json .= "    },\n"; 	
    //     }
    // } while (mssql_next_result($query2));

    // // Clean up
    // //mssql_free_result($version);
    // mssql_free_result($query2);
    // $json = substr($json, 0, -2);
    // $json .= "]";
    // $json .= "}";
    // return $json;
    "Cases".to_string()
}

fn getLastRegisteredHours() {
    // MSSQL and print it.
    // $query = mssql_query('
    // SELECT TOP 20 /*  [ServiceCaseId] */
    //     [Employee].[Name]floo
    //         ,[ServiceCaseTime].[TimeUsed]
    //         ,[InvoiceText]
    //     ,[Done]
    //     ,[Customer].[Name]
    //     ,[Headline]

    // FROM [RIS].[dbo].[ServiceCaseTime]
    // JOIN [RIS].[dbo].[Employee] ON (ServiceCaseTime.DoneBy =  Employee.Id)
    // JOIN [RIS].[dbo].[ServiceCase] ON (ServiceCaseTime.ServiceCaseId = ServiceCase.Id)
    // JOIN [RIS].[dbo].[Customer] ON (ServiceCase.CustomerId = Customer.Id)
    // order by ServiceCaseTime.Id DESC
    // ');

    // print '<table >';
    // print '<tr>';
    // print '<th><b>&nbsp;</b></th>';

    // print '<th><b>Navn</b></th>';
    // print '<th><b>Tid</b></th>';
    // print '<th><b>Fakturatekst</b></th>';
    // print '<th><b>Datostempel</b></th>';
    // print '<th><b>Kunde</b></th>';
    // print '<th><b>Tilh&oslash;rende sag</b></th>';
    // print '</tr>';
    // $j=0;

    // do {
    //     while ($row = mssql_fetch_row($query)) {
    //     $j++;
    //     print "<tr><td>$j</td>\n";
    //         for ($i = 0; $i <= 5; $i++) {
    //         print "<td class=r$i>";
    //         if ($i == 3 ) {
    //             echo date("m/d", strtotime($row[$i]));
    //         } else {
    //             print $row[$i];
    //         }
    //         print "</td>\n";
    //         }
    //     print "</tr>\n";
    //     }
    // } while (mssql_next_result($query));

    // mssql_free_result($query);
}

fn getHoursPerCustomer() {
    // $nu = date("Y-m-d", mktime(0, 0, 0, date('m'), date(1), date('Y')));
    // $kunder[]=0;
    // $txtquery="
    // select 
    //         Sum(ServiceCaseTime.TimeUsed) as Total,
    // SUBSTRING(Customer.Name, 1,21) AS Kunde
    //         from [ServiceCaseTime]
    //         join [ServiceCase] on (ServiceCaseTime.ServiceCaseId = ServiceCase.Id)
    //         join [Customer] on (ServiceCase.CustomerId = Customer.Id)
    //         where [Done] > '".$nu."'
    //         GROUP BY SUBSTRING(Customer.Name, 1,21)
    //         Order By Total DESC
    // ";
    // $query2 = mssql_query($txtquery);

    // $j = 0;
    // $db_found=False;
    // $db_nm[] = 0;
    // do {
    //     while ($row = mssql_fetch_row($query2)) {
    //     $j++;
    //     $kunde[$j][0]=$row[0];
    //     $kunde[$j][1]=$row[1];
    //     }
    // } while (mssql_next_result($query2));
    // mssql_free_result($query2);

    // return $kunde;
}

fn getHoursPerCustomerAsJson() {
    // Wait what? That's just the above function wrapped?
    // $json = '';
    // $json .= "{\n";
    // $json .= "    \"timer\": [";
    // foreach (getHoursPerCustomer() as $i => $value)  {
    //     $json .= "{\n";
    //     $json .= "            \"kunde\": \"$value[1]\",";
    //     $json .= "            \"timer\": $value[0]";
    //     $json .= "},\n";
    // }
    // $json = substr($json, 0, -2);
    // $json .= "    ]\n";
    // $json .= "}\n";

    // return $json;
}

fn getOpenCases() {
    // Retrieve Case Heads
    // $query = mssql_query('SELECT [Created], [Headline] ,[kontakter].Name ,[CustomerAgreement], [Deadline], sager.[Id]
    // FROM [dbo].[ServiceCase] AS sager
    //     LEFT JOIN [dbo].[CustomerContact] as kontakter
    //     ON [sager].CustomerContactId=[kontakter].Id
    // WHERE ([State] = 2 or [State] = 4) ORDER BY [Deadline]'); // and [Headline] NOT LIKE \'%Hvid%\'');
    // $output =  "{\n";
    // $output .= "    \"cases\": [\n";
    // do {
    //     // 0 created
    //     // 1 headline
    //     // 2 contact
    //     // 3 agreement
    //     // 4 deadline
    //     // 5 id
    //     while ($row = mssql_fetch_row($query)) {
    //         $created = date("Y-m-d H:i", strtotime($row[0]));
    //         $deadline = date("Y-m-d H:i", strtotime($row[4]));
    //         $agreement = preg_replace('#\s+#',', ',trim($row[3]));
    //         $headline = preg_replace( '#"+#','` ',$row[1]);
    //         //DebugDisplayVar($headline);
    //         $output .= "            {\n";
    //         $output .= "            \"id\": $row[5],\n";
    //         $output .= "            \"created\": \"$created\",\n";
    //         $output .= "            \"headline\": \"$headline\",\n";
    //         $output .= "            \"contact\": \"$row[2]\",\n";
    //         $output .= "            \"agrement\": \"$agreement\",\n";
    //         $output .= "            \"deadline\": \"$deadline\"\n";
    //         $output .= "            },\n";
    //     }
    // } while (mssql_next_result($query));
    // mssql_free_result($query);
    // $output = substr($output, 0, -2);
    // $output .= "    ]\n";
    // $output .= "}\n";
    // return $output;
}


fn getCaseDetails() {
    // Show Single Case
    // $query = mssql_query(
    // $select="SELECT [Created], [Headline] ,[kontakter].Name ,[CustomerAgreement], [Deadline], CONVERT(TEXT, [Description])
    //  FROM [dbo].[ServiceCase] AS sager
    // 	INNER JOIN [dbo].[CustomerContact] as kontakter
    // 	ON [sager].CustomerContactId=[kontakter].Id
    //   WHERE [sager].Id = '$sagsid'"); // and [Headline] NOT LIKE \'%Hvid%\'');

    // print '<table >';
    // do {
    // 	while ($row = mssql_fetch_row($query)) {
    // 	print "<tr><td>Sag oprettet: ".date("Y-m-d H:i", strtotime($row[0]))."</td</tr>";
    // 	print "<tr><td>Deadline: ".date("Y-m-d H:i", strtotime($row[4]))."</td</tr>";
    // 	print "<tr><td>".$row[1]."</td</tr>";
    // 	print "<tr><td>Kontakt: ".$row[2]."</td</tr>";
    // 	print "<tr><td>Status/aftalt: ".$row[3]."</td</tr>";
    // 	print "<tr><td>Beskrivelse:</td</tr>";
        
    // 	print "<tr><td>".$row[5]."</td</tr>";
    // 	}
    // } while (mssql_next_result($query));
    // mssql_free_result($query);
}


fn getPassedDueCases() {
    // return cases where deadline has been crossed

    // $query2 = mssql_query('
    // SELECT [Customer].[Name]   -- Kundens navn
    //     -- ,[Created]           -- Sag oprettet
    //     ,[Deadline]          -- 
    //     ,[Headline]          -- Overskrift
    //     --,[Description]     -- Sagsbeskrivelse
    //     ,[Employee].[Name]   -- Ansvarlige konsulent
    //     ,[Updated]		   -- Seneste opdatering af sagen
    //     --,[SolveHow]        -- Løsningsbeskrivelse
    //     ,[CustomerAgreement] -- Aftalt med kunden at
    
    // FROM [RIS].[dbo].[ServiceCase] 
    // JOIN [RIS].[dbo].[Customer] ON (ServiceCase.CustomerId = Customer.Id)
    // JOIN [RIS].[dbo].[Employee] ON (ServiceCase.ResponsibleEmployeeId =  Employee.Id)
    
    // where state = 4 -- 4 er overskredet
    // order by created asc -- asc er mest overskrevet først
    // ');
    // $otext = '';
    // $otext .= '<table>';
    // $otext .= '<tr>';
    // $otext .= '<th><b>Kunde</b></th>';
    // // $otext .= '<th><b>Oprettet</b></th>';
    // $otext .= '<th><b>Deadline</b></th>';

    // $otext .= '<th><b>Emne</b></th>';
    // $otext .= '<th><b>Uansvarlige konsulent</b></th>';
    // $otext .= '<th><b>Sidst opdateret</b></th>';
    // $otext .= '<th><b>Aftalt med kunden</b></th>';
    // $otext .= '</tr>';
    // $j = 0;
    // do {
    //     while ($row = mssql_fetch_row($query2)) {
    //     $j++;
    //     $otext .= '<tr>';
    //         for ($i = 0; $i <= 5; $i++) {
    //         $otext .= '<td>';
    //         if ($i == 1 or $i ==7 or $i == 4) {
    //             $otext .= date("d. M", strtotime($row[$i]));
    //         } else {
    //             $otext .= $row[$i];
    //         }
    //         $otext .= '</td>'; 	
    //         }
    //     $otext .= '</tr>';
    //     }
    // } while (mssql_next_result($query2));

    // // Clean up
    // //mssql_free_result($version);
    // mssql_free_result($query2);

    // $otext .= '</table>';

    // if ($j == 0) {
    //     print ("<h2>Wow! Ingen overskredne :-)</h2>");
    // } else {
    // print '<h2>'.$j.' overskredne sager, men hvem t&aelig;ller?</h2>';
    // // print ($otext);
    // }
    // // return $json;
}

fn getHoursPerEmployee() {
    // $nu = date("Y-m-d", mktime(0, 0, 0, date('m'), date(1), date('Y')));

    // $txtquery="
    // SELECT
    //     Sum([ServiceCaseTime].[TimeUsed]) As Total
    //     ,[Employee].[Name]

    // FROM [RIS].[dbo].[ServiceCaseTime]
    // JOIN [RIS].[dbo].[Employee] ON (ServiceCaseTime.DoneBy =  Employee.Id)
    // JOIN [RIS].[dbo].[ServiceCase] ON (ServiceCaseTime.ServiceCaseId = ServiceCase.Id)

    // Where [Done] > '".$nu."'

    // Group by [Employee].Name
    // -- order by [TOTAL] desc
    // order by [Employee].[Name] asc
    // ";
    // $query2 = mssql_query($txtquery);
    // $json = '';
    // $json .= "{\n";
    // $json .= "    \"timer\": [";
    // do {
    //     while ($row = mssql_fetch_row($query2)) {
    //     $json .= "{";
    //     $json .= '"employee": "'.$row[1].'",';
    //     $json .= '"hours": '.$row[0].'';
    //     $json .= '},';
    //     }
    // } while (mssql_next_result($query2));
    // $json = substr($json, 0, -1);
    // $json .= ']';
    // $json .= '}';
    // mssql_free_result($query2);
    // return $json;
}
