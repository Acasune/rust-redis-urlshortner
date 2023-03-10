import { Button, FormLabel, HStack, Input, VStack } from '@chakra-ui/react'
import { useRouter } from 'next/navigation'
import { useState } from 'react'
import { useForm } from 'react-hook-form'
import { addUrl } from 'service/post-url'

const PostUrlContainer = () => {
  const [url, setUrl] = useState('')
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm()
  const router = useRouter()
  const onSubmit = async (data: any) => {
    try {
      const ret = await addUrl(
        { user_name: 'Yamada taro', url: data.url },
        { apiRootUrl: 'http://localhost:8080/' },
      )
      router.refresh()
    } catch (err: unknown) {
      if (err instanceof Error) {
        window.alert(err.message)
      }
    }
  }
  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <VStack>
        <FormLabel htmlFor="name">Register new URL</FormLabel>
        <HStack w="30vw">
          <Input id="url" placeholder="Add new URL" {...register('url')} />
          <Button colorScheme="blue" type="submit">
            Post
          </Button>
        </HStack>
      </VStack>
    </form>
  )
}

export default PostUrlContainer
