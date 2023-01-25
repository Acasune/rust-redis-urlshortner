import { Button } from '@chakra-ui/react'
import { delUrl } from 'service/delete-url';

export type DelUrlButtonProps = {
  hash:string
}

const DelUrlButton = ({hash}:DelUrlButtonProps) => {
  const onClick = async (data: any) => {
    try {
      const ret = await delUrl({hash},{apiRootUrl:'http://localhost:8080/'})
    } catch (err:unknown) {
      if (err instanceof Error) {
        window.alert(err.message)
      }
    }
  }
  return (
    <Button colorScheme='blue' onClick={onClick}>Delete</Button>
  )
}

export default DelUrlButton