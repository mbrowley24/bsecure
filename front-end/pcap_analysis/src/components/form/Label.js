import {} from 'react'




const Label = (props) =>{

    return(
        <>
            <label
                htmlFor={props.htmlFor}
                className={props.className}
            >{props.text}</label>
        </>
    )
}

export default Label;