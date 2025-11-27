import './App.css'
import { useState } from 'react'
import { HashSubmitButton } from './comps/hashSubmitButton/hashSubmitButton'
import { HashInputField } from './comps/hashInputField/hashInputField'


function App() {
  const [file, setFile] = useState<File | null>(null);
  console.log(file);
  const handleClick = async () => {
    try {
      const response = await fetch('http://localhost:8000/api/data'); //need backend server running
      const data = await response.json();
      console.log(data);
    } catch (error) {
      console.error('Error fetching data:', error);
    }
  }

  return (
    <>
      <div>
        <h1>hashcontrol</h1>
      </div>

      <div className="main-container">
        <HashInputField onFileChange={setFile}/>
        <HashSubmitButton onClick={handleClick}/>
      </div>
    </>
  )
}

export default App


/*
Frontend -> debian server
rainbow tables -> ..
Rust funciton -> threads 
returns 


input of the hashes ig.
multiple hashes at once
do mailu hash => plaintext



*/
