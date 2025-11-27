
interface HashInputFieldProps {
    onFileChange: (file: File) => void;
}

export const HashInputField = ({onFileChange}: HashInputFieldProps) => {


    return (
        <input 
            type="file" 
            accept=".csv" 
            onChange={(e) => onFileChange(e.target.files![0])}
        />);
};