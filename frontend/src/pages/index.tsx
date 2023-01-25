import useSWR from 'swr'
import { ApiContext, Data } from '@/types/data'
import Head from 'next/head'
import { Button, FormLabel, HStack, Input, Table, TableCaption, TableContainer, Tbody, Td, Tfoot, Th, Thead, Tr, VStack } from '@chakra-ui/react'

export default function Home() {
  const {ret, isLoading} = useGetAllUrls()
  return (
    <>
      <Head>
        <title>Create Next App</title>
        <meta name="description" content="Generated by create next app" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
      <main>
        <VStack>        
          <FormLabel htmlFor="name">Register new URL</FormLabel>
          <HStack w="30vw">
            <Input placeholder='Add new URL' />
            <Button  colorScheme='blue'>Submit</Button>
          </HStack>
        </VStack>

        <TableContainer>
          <Table variant='simple'>
            <Thead>
              <Tr>
                <Th>Hash vals</Th>
                <Th>Raw urls</Th>
                <Th>Actions</Th>
              </Tr>
            </Thead>
            <Tbody>
            {ret.map((item, idx) => (
              <Tr><Td>{item.hashed}</Td><Td>{item.url}</Td><Td><Button colorScheme='blue'>Delete</Button></Td></Tr>
              ))}
            </Tbody>
          </Table>
        </TableContainer>
      </main>
    </>
  )
}

type UseGetAllUrls = {
  ret: [{hashed:string, url:string}]
  isLoading: boolean
  isError: boolean
}

const useGetAllUrls= () : UseGetAllUrls => {
 
  const fetcher = (url:string):Promise<any> => fetch(url).then(res =>{
    console.log(res)
    return res.json()
  })
  const path :string = process.env.API_BASE_URL || 'http://localhost:8080/'
  
  const {data, error} = useSWR<Data>(
    path, fetcher
  )

  return {
    ret: data ? data.urls :[{hashed:'',url:''}],
    isLoading: !error && !data,
    isError: !!error, }
}

