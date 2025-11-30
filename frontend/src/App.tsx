import './App.css'
import { useState } from 'react'
import { HashSubmitButton } from './comps/hashSubmitButton/hashSubmitButton'
import { HashInputField } from './comps/hashInputField/hashInputField'


function App() {
  const [file, setFile] = useState<File | null>(null);
  console.log(file);
  
  const handleUpload = async () => {
    if (!file) return;

    const formData = new FormData();
    formData.append("csv", file);

    const response = await fetch("http://localhost:8000/api/upload-csv", {
      method: "POST",
      body: formData,
    });

    const data = await response.json();
    console.log("Response:", data);
};

  return (
    <>
      <div>
        <h1>hashcontrol</h1>
      </div>

      <div className="main-container">
        <HashInputField onFileChange={setFile}/>
        <HashSubmitButton onClick={handleUpload}/>
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
