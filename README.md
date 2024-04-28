# Daily News Roundup

## Preface

At the beginning of 2024, I wanted to get my iPhone screentime down to an average of 1 hour daily. I had a lot of stuff I wanted to do this year (finishing my MSc, learning an instrument, all alongside doing good quality work with a healthy work-life balance), so there was a pull factor in wanting to be as productive as possible, plus there's a lot of stuff out there on how the way our smartphone technology (and specifically social media) are set up wrecks havoc with our dopamine system.

I did manage this and it was definitely a net positive, but the downside was I didn't have any idea what was going on in the society I lived in anymore -- I was totally uninformed, which I don't think is a cool or interesting trait to have. There were lower effort ways of squaring that circle, but this way I had a project in the new programming language I was learning.

## What is this?

A daily email in my inbox, which contains a headline (as a hyperlink) and a short summary of the following:

- The top story on the Financial Times UK page.
- Today's Bandcamp Album of the Day.
- The top story on HackerNews at the time of running (using the OpenAI API to summarise the article in a short paragraph).
- The most recent story from Novara Media, a UK based political news website.

## Installation

There's a few steps to do before cloning this repo, would recommend doing the following in the order printed:

---

On your local machine, open a terminal and connect via SSH:

```
ssh root@your_linodes_ip
```

Then, install and update all the non-Rust related packages required for installing and running the package. Run the following three commands, one at a time:

```
apt update && apt upgrade -y
sudo apt install build-essential curl git pkg-config libssl-dev -y
```

Handle any conflicts by taking the package managers version just to be safe, and if you're prompted to restart the kernel, just do it.

Install Rust (pick the default installation):

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, add Rust to your shell’s PATH:

```
source $HOME/.cargo/env
```

Install git on the Linux node if it's not already installed:

```
apt install git -y
```

Now you can clone this repo! Really, you can do this first, but I prefer having everything set up beforehand.

```
git clone https://github.com/fraserwat/newsround.git && cd newsround
```

Compile the package into a binary.

```
cargo build --release
```

There are some environment variabels that need to be set. Create a .env file (with the above commands you should already been in the apps root folder) with the following variables (you'll need to set up your own account on OpenAI and Mailgun, and EMAIL_TO can just be your personal email):

```
OPENAI_API_KEY=xyzxyzxyz
MAILGUN_API_KEY=abc123
MAILGUN_DOMAIN=sandbox1234567890.mailgun.org
EMAIL_FROM="Your Name <yourname@mailgundomain1234567890.mailgun.org>"
EMAIL_TO=youremail@gmail.com
```

Give it a quick test run, to make sure everything works ok. The dotenv loader at the start of `main()` assumes that you have saved this in the project root (i.e. `/newsround/.env`).

```
./target/release/newsround
```

If it all checks out, you're nearly ready to set up the cron schedule. If you're not super picky on when it sends you can skip ahead, but there's some annoying timezone defaults (at the time of writing it's British Summer Time, meaning everything is an hour out).

```
sudo timedatectl set-timezone Europe/London
sudo timedatectl set-ntp true
```

If you're not in the UK, you'll need to find your own timezone with `timedatectl list-timezones`. Verify your changes stuck (`timedatectl`), and you should see something like the following:

```
               Local time: Mon 2024-04-22 09:50:35 BST
           Universal time: Mon 2024-04-22 08:50:35 UTC
                 RTC time: Mon 2024-04-22 08:50:34
                Time zone: Europe/London (BST, +0100)
System clock synchronized: yes
              NTP service: active
          RTC in local TZ: no
```

The important bit is the "System clock synchronized: yes". Now you can open up your cron scheduler (`crontab -e`), and set up a new job. The below is daily at 7AM, giving me something to read after the gym during breakfast! 🙂

```
0 7 * * * /root/newsround/target/release/newsround >> /var/log/newsround.log 2>&1
```

Might want to mess around with the cron as a duplicate line to make sure that it's actually running, but then you're good to go!

## To-Do's

- [ ] Rewrite the API calls for various websites (Bandcamp, Financial Times, Novara, Hacknernews) to run in parallel.
- [ ] Prompt engineering for better subject lines.
- [ ] Refactor so you get the latest post on the Bandcamp Daily blog, and not today's Album of the Day (I just find the former more interesting).
