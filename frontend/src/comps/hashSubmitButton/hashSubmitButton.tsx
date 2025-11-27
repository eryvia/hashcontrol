

interface HashSubmitButtonProps {
    data?: Array<string>;
    onClick: () => void;
}

export function HashSubmitButton({ onClick, data }: HashSubmitButtonProps) {
    console.log("HashSubmitButton data:", data);
  
    return (
        <button
            onClick = {() => onClick}
        >
            Submit Hashes
        </button>
  )
}