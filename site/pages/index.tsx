import Head from 'next/head'
import styles from '../styles/Home.module.css'
import { getChartData } from '../data/chartData'
import { CompilerMode, ProfileMode, System } from '../data/types'
import { LineChartX, LineChartXProps} from '../components/linechart'

export default function Home({chartData}) {
  return (
    <div className={styles.container}>
      <Head>
        <title>Are We Fast Yet</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>
          Benchmarking the Rust compiler
        </h1>

        <div className={styles.grid}>
          {Object.keys(chartData).map(repo => {
            const props = {
              chartData: chartData[repo],
              profile_modes: [ProfileMode.Clean],
              compiler_modes: [CompilerMode.Debug, CompilerMode.Release],
              systems: [System.TwoCores, System.FourCores, System.EightCores, System.SixteenCores],
            };
            return <LineChartX {...props}/>
          })
          }
        </div>
      </main>

      <footer className={styles.footer}>
        <a
          href="https://vercel.com?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
          target="_blank"
          rel="noopener noreferrer"
        >
          Powered by{' '}
          <img src="/vercel.svg" alt="Vercel Logo" className={styles.logo} />
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
