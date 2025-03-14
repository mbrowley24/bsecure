import {} from 'react';





const Button = (props) =>{


    return (
        <>
            <button
             className={props.className}
             disabled={props.disabled? props.disabled : false}
             type={props.type? props.type: 'submit'}
             onClick={props.action}
            >
                {props.text}
            </button>
        </>
    )
}

export default Button;