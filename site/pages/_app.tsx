import '../styles/globals.css'
import 'bootstrap/dist/css/bootstrap.min.css';
import type { AppProps /*, AppContext */ } from 'next/app'

function MyApp({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />;
}


export default MyApp
