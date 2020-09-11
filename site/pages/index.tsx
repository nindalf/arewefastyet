import Head from 'next/head'
import styles from '../styles/Home.module.css'
import {getAllPerfData} from '../data/profiling'
import LineChart from '../components/linechart'

export default function Home({allPerfData}) {
  return (
    <div className={styles.container}>
      <Head>
        <title>Are We Fast Yet</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>
          Welcome to Are We Fast Yet!
        </h1>

        <div className={styles.grid}>
          {Object.keys(allPerfData).map(repo => {
            return <LineChart {...allPerfData[repo]}/>
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
  const allPerfData = getAllPerfData()['0']
  return {
    props: {
      allPerfData
    }
  }
}
