# Arewefastyet

This repo is split into 3 parts

- `site` powers the dashboard on [arewefastyet.rs](https://arewefastyet.rs/), as well as the [FAQ](https://arewefastyet.rs/faq/)
- `data` has the data powering those dashboards, stored in JSON.
- `cmd` is a CLI written in Rust that collects that data.

If you'd like to reproduce these benchmarks, run this

```
git clone https://github.com/nindalf/arewefastyet
cd arewefastyet/
./collect_samples.sh
```

## Contributing

The site doesn't do very well on mobile, so any contribution there are welcome. To get started

```
git clone https://github.com/nindalf/arewefastyet
cd arewefastyet/
npm install && npm run dev
```

For now, contributions of benchmarks on different kinds of hardware is out of scope. 
