import Head from 'next/head'
import styles from '../styles/Home.module.css'
import { getChartData } from '../data/chartData'
import { AppContainer } from '../components/appcontainer'

export default function Home({ chartData }) {
  return (
    <div className={styles.container}>
      <Head>
        <title>Are We Fast Yet</title>
        <link rel="icon" href="/favicon.ico" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0"></meta>
      </Head>

      <main className={styles.main}>
        <h3 className={styles.title}>
          Benchmarking the Rust compiler
        </h3>
        <div className={`${styles.card} ${styles.faq}`}>Arewefastyet measures how long the Rust compiler takes to compile common Rust programs.<br />Lower is better. Check out the <a href="/faq">FAQ</a></div>
        <AppContainer {...chartData} />
      </main>

      <footer className={styles.footer}>
        <a
          href="https://github.com/nindalf/arewefastyet"
          target="_blank"
          rel="noopener noreferrer"
        >
          <img src="/GitHub-Mark-64px.png" />
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
