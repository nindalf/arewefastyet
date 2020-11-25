import Head from 'next/head'
import styles from '../styles/Home.module.css'
import { getChartData } from '../data/chartData'
import { CompilerMode, ProfileMode, System } from '../data/types'
import { LineChartX, LineChartXProps } from '../components/linechart'

export default function Home({ chartData }) {
  return (
    <div className={styles.container}>
      <Head>
        <title>Are We Fast Yet</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h2 className={styles.title}>
          Benchmarking the Rust compiler
        </h2>

        <h3>Check out the <a href="/faq">FAQ</a></h3>

        <div className={styles.grid}>
          {Object.keys(chartData).map(repo => {
            const props: LineChartXProps = {
              chartData: chartData[repo],
              profile_modes: [ProfileMode.Clean],
              compiler_modes: [CompilerMode.Debug, CompilerMode.Release],
              systems: [System.TwoCores, System.FourCores, System.EightCores, System.SixteenCores],
            };
            return <LineChartX {...props} key={chartData[repo].repo.name}/>
          })
          }
        </div>
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
  const chartData = getChartData()
  return {
    props: {
      chartData
    }
  }
}
