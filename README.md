# TEA pal
Query TNEA and JOSAA Counselling Details.

## Usage GIF (453kb):
![alt GIF of Webpage](https://i.ibb.co/pd9vPwQ/tea-pal-readme.gif)

# How to use?
- Live @ https://tea-pal-skndash96.koyeb.app/
- Fork the repo and setup rust (if needed) and hit `cargo run` on CLI.
(or)
- Deal with the docker image: https://hub.docker.com/repository/docker/skndash96/tea-pal/general

# Info
- The SQLite Database contains data of 2023 TNEA Counselling results of various students in Round 1.
- Contains results off cutoff range  177 to 200 (maximum mark).
- The SQLite Database also contains about 350k records of JOSAA counselling OR,CR details of years 2016-23.
- Contact me for the database copy or in csv format.
- The TNEA and JOSAA 2023 data are fetched by me from the official sites while JOSAA 2016-22 are taken from kaggle site.

# API
## TNEA: `/api/tnea?`
- cutoff=[number]
- rank=[number]
- limit=[number]
- college=[text]
- coll_code=[number]
## JOSAA: `/api/josaa?`
- rank=[number]    (lower bound of rank)
- name=[text]
- course=[text]
- quota=[text] (HS,OS,AI,AP,JK,GO,LA)
- seat=[text] (OPEN,EWS,OBC-NCL,SC,ST,PwD)
- gender=[text] (neutral,female,NA)
- year=[number] (2016 to 2023)
- round=[number] (1 to 6)
