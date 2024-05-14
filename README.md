# Daily News Roundup

## What is this?

A daily email in my inbox, which contains a headline (as a hyperlink) and a short summary of the following:

- The top story on the Financial Times UK page.
- Today's Bandcamp Album of the Day.
- The top story on HackerNews at the time of running (using the OpenAI API to summarise the article in a short paragraph).
- The most recent story from Novara Media, a UK based political news website.

## Preface

In 2024, I wanted to get my iPhone screentime down to ~1 hour daily. There's a lot of stuff I want to do this year (finishing a MSc, learning an instrument, doing good quality work with a healthy work-life balance), so theres a pull factor in wanting to be as productive as possible, plus I'd been reasding a lot about the impact of smartphone use (specifically social media!) on attention spans, dopamine receptors etc and I dunno, there's just stuff in the world I don't need to see first thing every morning.

I **did** manage this and it was a huge net positive, but the downside was I didn't have any idea what was going on in the society I lived in -- totally uninformed! Which I don't think is a cool or interesting trait to have. There's probably lower effort ways of squaring that circle, but this way I got to build project in the new programming language I was learning.

### What did I learn?

- A lot of Rust experience! Including its (sometimes annoying!) type system, what needs to be a public / private function (I never had to worry about in Python), working with API's, JSON, working with HTML in Rust, etc.
- Parallelism! Basically all the runtime in this program is spent waiting on API calls so I was able to speed it up by 28% by running a lot of them in parallel. Much easier to do this in Rust than Python (where I've always found parallelism to be sort of an afterthought?).
- Prompt engineering for summarising text in varying degrees of berevity.

## Installation

There's a few steps to do before cloning this repo, would recommend doing the following in the order printed:

---

On your local machine, open a terminal and connect via SSH:

```
ssh root@your_linodes_ip
```

Install and update non-Rust related packages required for installing and running the package. Run the following commands, one at a time:

```
apt update && apt upgrade -y
sudo apt install build-essential curl git pkg-config libssl-dev -y
```

Handle any conflicts by taking the package managers version just to be safe, and if you're prompted to restart the kernel, just do it.

Install Rust (pick the default installation):

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, add Rust to the shell’s PATH:

```
source $HOME/.cargo/env
```

Install git on the Linux node if it's not already installed:

```
apt install git -y
```

Now you can clone this repo! Can really be done first, but I prefer having everything set up beforehand.

```
git clone https://github.com/fraserwat/newsround.git && cd newsround
```

Compile the package into a binary.

```
cargo build --release
```

There are some environment variabels that need to be set. Create a .env file (with the above commands you should already been in the apps root folder) with the following variables (set up your account on OpenAI and Mailgun, EMAIL_TO can just be your personal email):

```
OPENAI_API_KEY=xyzxyzxyz
MAILGUN_API_KEY=abc123
MAILGUN_DOMAIN=sandbox1234567890.mailgun.org
EMAIL_FROM="Your Name <yourname@mailgundomain1234567890.mailgun.org>"
EMAIL_TO=youremail@gmail.com
```

Give it a test run to make sure everything works ok. The dotenv loader at the start of `main()` assumes that you have saved this in the project root (i.e. `/newsround/.env`).

```
./target/release/newsround
```

If it all checks out, you're nearly ready to set up the cron schedule. There's some annoying timezone defaults (at the time of writing it's British Summer Time, meaning everything is an hour out from UTC).

```
sudo timedatectl set-timezone Europe/London
sudo timedatectl set-ntp true
```

If not in the UK, find your timezone with `timedatectl list-timezones`. Verify changes (`timedatectl`) and you should see something like the following:

```
               Local time: Mon 2024-04-22 09:50:35 BST
           Universal time: Mon 2024-04-22 08:50:35 UTC
                 RTC time: Mon 2024-04-22 08:50:34
                Time zone: Europe/London (BST, +0100)
System clock synchronized: yes
              NTP service: active
          RTC in local TZ: no
```

The important bit is the "System clock synchronized: yes". Open up the cron scheduler (`crontab -e`) and set up a new job. The below is 7AM daily, giving me something to read after the gym during breakfast! 🙂

```
0 7 * * * /root/newsround/target/release/newsround >> /var/log/newsround.log 2>&1
```

Might want to mess around with the cron on a new line to make sure it's actually running, but you're good to go!

## To-Do's

- [x] ~~Rewrite website API calls to run in parallel.~~ Done, with a 28% drop in execution time!! 🔥🔥🔥
- [ ] Prompt engineering for better subject lines.
- [ ] Refactor for latest post on the Bandcamp Daily blog, not today's Album of the Day (I just find the former more interesting).
