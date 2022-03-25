import { Spinner, Table, Tbody, Th, Thead, Tr, VStack, Td, Flex, Button, Text, HStack, Divider, IconButton, Spacer } from '@chakra-ui/react';
import axios from 'axios';
import { useCallback, useEffect, useRef, useState } from 'react';
import { useQuery } from 'react-query';
import { BsImageFill } from "react-icons/bs";
import { IoCloseSharp } from 'react-icons/io5';
import { css, before } from 'glamor';

export default function Home() {
  const ref = useRef();
  const [files, setFiles] = useState([]);

  const handleFileChange = (e) => {
    console.log(e.target.files);
    setFiles(Array.from(e.target.files).map(f => {
      return {
        progress: 0,
        file: f
      }
    }));

    // const formData = new FormData();

    // formData.append('file', e.target.files[0]);

    // axios.put(`http://localhost:8080?name=${e.target.files[0].name}&ctype=${e.target.files[0].type}`, formData).then((res) => {
    //   console.log("done: ");
    // });
  }

  return (
    <VStack h="100vh" w="100vw" justifyContent="center" alignItems="center" spacing={0} bg="#050505">
      <VStack spacing={4} minH="200px" w={["300px", "500px"]} bg="#090909" justifyContent={"center"} alignItems={"center"} borderRadius={15} pt={4} pb={4}>
        <Text fontWeight={"bold"} color="white" fontSize={"2xl"}>Upload your media</Text>
        {
          files.length
            ? <UploadFilesButton files={files} setFiles={setFiles} />
            : <ChooseFilesButton inputRef={ref} />
        }
        {
          files.map((f) => {
            return <FileEntry file={f} setFiles={setFiles} />
          })
        }
      </VStack>
      <input onChange={handleFileChange} style={{ display: "none" }} ref={ref} type="file" multiple></input>
    </VStack>
  )
}

function UploadFilesButton({ files, setFiles }) {
  const [clickedOnce, setClickedOnce] = useState(false);

  const handleClick = useCallback(() => {
    if (!clickedOnce) {
      setClickedOnce(c => !c);
      return;
    }

    for (let i = 0; i < files.length; i++) {
      const config = {
        onUploadProgress: progressEvent => {
          let { progress } = files[i];
          progress = (progressEvent.loaded / progressEvent.total) * 100;

          setFiles((current) => {
            const clone = [...current];
            clone[i].progress = progress;

            return clone;
          });
        }

      }
      let formData = new FormData();
      formData.append("file", files[i].file);

      axios.put(`http://10.0.0.29:8080?name=${files[i].file.name}&ctype=${files[i].file.type}`, formData, config).then((res) => {
        console.log(res.data);
      }).catch(err => {
        console.log("error: ", err.message);
      });
    }

  }, [clickedOnce]);

  useEffect(() => {
    if (clickedOnce) {
      const t = setTimeout(() => {
        setClickedOnce(false);
      }, 5000);

      return () => {
        console.log("clear");
        clearTimeout(t);
      }
    }
  }, [clickedOnce]);

  return <Button
    onClick={handleClick}
    w="90%"
    br={15}
    bg={clickedOnce ? "#3c5c3c" : "#4c3280"}
    h="40px"
    color={"white"}
    _hover={{ bg: "#5a8a5a" }}
    _focus={{}}
    _active={{}}>
    {!clickedOnce ? "Upload 🚀" : "Click to confirm"}
  </Button>
}

function ChooseFilesButton({ inputRef }) {
  return <Button
    onClick={() => inputRef.current.click()}
    w="90%"
    br={15}
    bg="#4c3280"
    h="40px"
    color={"white"}
    _hover={{ bg: "#6a45b3" }}
    _focus={{}}
    _active={{}}>
    Choose files to upload
  </Button>
}


function FileEntry({ file: { file, progress }, setFiles }) {
  return <HStack w="90%" {...before({
    content: "1",
    color: "transparent",
    position: "absolute",
    left: 0,
    bottom: 0,
    height: "5px",
    borderBottomColor: "red",
    width: `${progress}%`,
    borderRadius: 4,
    borderBottom: "4px solid #3c5c3c",
  })}
    className="test"
    spacing={2} h="45px"
    bg="#121212"
    borderRadius={5}
    pl={3} pr={2}
    position="relative"
    zIndex={1}
  >
    <BsImageFill style={{minWidth: "1rem", minHeight: "1rem"}} color="white" />
    <Divider orientation='vertical' h="48%" />
    <Text color="white" isTruncated>{file.name}</Text>
    <Spacer />
    {
      0 < progress && progress < 100 ?
        <Spinner color="white" size="sm" />
        :
        <IconButton
          icon={<IoCloseSharp color={"white"} />}
          h="30px"
          minW="30px"
          bg="transparent"
          _hover={{ bg: "#181818" }}
          _active={{}}
          _focus={{}}
          onClick={() => { setFiles((curr) => curr.filter(f => f.file.name !== file.name)) }} />
    }
  </HStack>
}
function parseDate(date) {
  let minutes = date.getMinutes();
  minutes = minutes < 10 ? `0${minutes}` : minutes;

  return `${date.getDate()}/${date.getMonth() + 1}/${date.getFullYear()} at ${date.getHours()}:${minutes}`;
}