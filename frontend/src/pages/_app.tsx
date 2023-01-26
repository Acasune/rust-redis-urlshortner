// 1. Import the extendTheme function
import {
  ChakraBaseProvider,
  ChakraProvider,
  extendTheme,
} from '@chakra-ui/react'
import { AppProps } from 'next/app'

// 2. Extend the theme to include custom colors, fonts, etc
const colors = {
  brand: {
    900: '#1a365d',
    800: '#153e75',
    700: '#2a69ac',
  },
}

const theme = extendTheme({ colors })

export default function App({ Component, pageProps }: AppProps) {
  return (
    <ChakraProvider theme={theme}>
      <Component {...pageProps} />
    </ChakraProvider>
  )
}
