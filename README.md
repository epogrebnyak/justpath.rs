# justpath.rs

What's on your `PATH`? 

## How to run

```bash
git clone https://github.com/epogrebnyak/justpath.rs.git
cd justpath.rs
cargo run
```

## Sample output 

Github Codespaces (February 2026) `PATH` has quite a few duplicates and some missing directories:

```
$ cargo run -q | grep ruby
 2 /usr/local/rvm/gems/ruby-3.4.7/bin (duplicate ×2)
 3 /usr/local/rvm/gems/ruby-3.4.7@global/bin (unique) → /usr/local/rvm/rubies/ruby-3.4.7/lib/ruby/gems/3.4.0/bin
 4 /usr/local/rvm/rubies/ruby-3.4.7/bin (duplicate ×3)
14 /home/codespace/.ruby/current/bin (duplicate ×3) → /usr/local/rvm/rubies/ruby-3.4.7/bin
27 /usr/local/rvm/gems/default/bin (duplicate ×2) → /usr/local/rvm/gems/ruby-3.4.7/bin
29 /usr/local/rvm/rubies/default/bin (duplicate ×3) → /usr/local/rvm/rubies/ruby-3.4.7/bin
```

```
$ cargo run -q | grep missing
26 /usr/local/sdkman/candidates/ant/current/bin (missing)
28 /usr/local/rvm/gems/default@global/bin (missing)
43 /home/codespace/.dotnet/tools (missing)
```

## Previous art

[Python version](https://github.com/epogrebnyak/justpath) is richer in flags until I learn [clap](https://docs.rs/clap/latest/clap/).