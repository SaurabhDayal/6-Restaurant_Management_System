select
  ac.account_id, ac.user_name,
  (ad.address_id, ad.city, ad.country, ad.street, ad.street_number) as "address!: Address",
  array_agg((p.post_id, p.title, p.body)) as "posts!: Vec<Post>"
from accounts as ac
join address as ad using(address_id)
join posts as p using(account_id) 
where ac.account_id = 1
group by ac.account_id, "address!: Address";


should do the trick and gives us

account       user_name               address!: Address	                         posts!: Vec
    1             	peter      	(1,city_1,country_1,street_1,1a)	     {"(1,"first post","my first post has not much text")",
                                                                          "(2,"second post","my second post has more text, but   not much more.")",
                                                                          "(3,"third post","third one is even shorter")"}
