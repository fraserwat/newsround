On your local machine, open a terminal and connect via SSH:
`ssh root@your_linodes_ip`

Install and update all the non-Rust related packages required for installing and running the package.

```
apt update && apt upgrade -y
sudo apt install build-essential curl git pkg-config libssl-dev -y
```

Handle any conflicts by taking the package managers version just to be safe, and if you're prompted to restart the kernel, just do it.

Install Rust (pick the default installation).

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

After installation, add Rust to your shell’s PATH:

`source $HOME/.cargo/env`

Install git on the Linux node.

`apt install git -y`

Clone the repo

`git clone https://github.com/fraserwat/newsround.git && cd newsround`

Compile the package into a binary.

`cargo build --release`

There are some environment variabels that need to be set. Create a .env file (with the above commands you should already been in the apps root folder) with the following variables:

```
OPENAI_API_KEY=xyzxyzxyz
MAILGUN_API_KEY=abc123
MAILGUN_DOMAIN=sandbox1234567890.mailgun.org
EMAIL_FROM="Your Name <yourname@mailgundomain1234567890.mailgun.org>"
EMAIL_TO=youremail@gmail.com
```

Give it a quick test run, to make sure everything works ok. The dotenv loader at the start of `main()` assumes that you have saved this in the project root (i.e. `/newsround/.env`).

`./target/release/newsround`

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
