import Head from 'next/head'
import styles from '../styles/Home.module.css'
import { Converter } from 'showdown'
import utilStyles from '../../styles/utils.module.css'

const faq_md = `
- What is being measured here?
	- Arewefastyet measures how long the Rust compiler takes to compile common Rust programs, not how fast compiled rust programs in general are. Over the last few years, [significant](https://pingcap.com/blog/rust-compilation-model-calamity#recent-work-on-rust-compile-times) [effort](https://blog.mozilla.org/nnethercote/2020/09/08/how-to-speed-up-the-rust-compiler-one-last-time/) has been put into optimization. Recent releases of the Rust compiler are significantly faster than releases from a year ago. There is more exciting work planned, like a new [debug backend](https://bjorn3.github.io/2020/09/28/progress-report-sep-2020.htm) and Profile Guided Optimization.
- What is Rust again? 
	- Rust is a language empowering everyone to build reliable and efficient software. More details [here](https://www.rust-lang.org/)
- Does Rust compile programs slowly?
    - Programming language design involves trade-offs. This [post](https://pingcap.com/blog/rust-compilation-model-calamity#recent-work-on-rust-compile-times) discusses the choices the Rust project made over the years.
- How can I improve compile times in my project?
	- You can try some these suggestions - [Rust Performance Book - Compile Times](https://nnethercote.github.io/perf-book/compile-times.html), [8 Solutions for Troubleshooting Your Rust Build Times](https://medium.com/@jondot/8-steps-for-troubleshooting-your-rust-build-times-2ffc965fd13e) and [How to alleviate the pain of Rust compile times](https://vfoley.xyz/rust-compile-speed-tips/). Or just wait for the next release of Rust; it's less than 6 weeks away!
- Why use wall time as a metric? Why not something else?
	- This [discussion](https://internals.rust-lang.org/t/what-is-perf-rust-lang-org-measuring-and-why-is-instructions-u-the-default/9815/4) touches on the options. Instruction count, the number of instructions executed by rustc during compilation is used by perf.rust-lang.org to track regressions in the compiler. But wall time is the metric that users care about.
- What hardware did you use? 
	- Virtual Machines with dedicated CPU
    - CPU model - Intel(R) Xeon(R) Gold 6140 CPU @ 2.30GHz
    - Number of cores - 2, 4, 8, 16
    - Memory - 4GB, 8GB, 16GB, 32GB. (none of the builds used more than 1 GB of memory, so memory wasn't a bottleneck).
    - OS - Ubuntu 20.04 LTS. No changes made apart from the packages installed by [./collect_samples.sh](https://github.com/nindalf/arewefastyet/blob/master/collect_samples.sh)
- Can I see the raw data?
    - Sure, it's in [./data](https://github.com/nindalf/arewefastyet/tree/master/data).
- Will the hardware change? Will alternate configurations be supported?
    - Adding support for more hardware is something I'm not considering right now. I did consider adding benchmarks from a typical laptop or from a workstation but I decided against it because of concerns around reproducibility (laptops throttle aggressively) and future access to the same hardware. A VM with a dedicated CPU has neither of those issues. 
- How to contribute to rust development?
	- If you're looking for Rust projects to contribute to, keep an eye on the "calls for participation" section in [This week in Rust](https://this-week-in-rust.org/), a newsletter. To contribute to Rust itself, check out 
- How to contribute to arewefastyet?
    - Contributions welcome to profiling code or frontend.
    - If there is a important rust repo that is different enough from the existing ones, open [an issue](todo). Importance is correlated with how widely used the tool (ripgrep, alacritty, rav1e) or library (hyper, serde, clap) is. If you'd like to add one, please follow the format in [./data/repos.json](https://github.com/nindalf/arewefastyet/blob/master/data/repos.json)
`;

export default function Home({ faqHtml }) {
  return (
    <div className={styles.container}>
      <Head>
        <title>Are We Fast Yet</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h2 className={styles.title}>
          FAQ
        </h2>
        <div className={styles.grid} dangerouslySetInnerHTML={{ __html: faqHtml }}/>
      </main>

      <footer className={styles.footer}>
        <a
          href="https://github.com/nindalf/arewefastyet"
          target="_blank"
          rel="noopener noreferrer"
        >
          Link to arewefastyet repo
        </a>
      </footer>
    </div>
  )
}

export async function getStaticProps() {
    const converter = new Converter();
    const faqHtml = converter.makeHtml(faq_md);
    return {
      props: {
        faqHtml
      }
    }
  }
  