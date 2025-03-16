import {} from 'react'



const Input = (props) =>{


    return (
        <>
            <input
                className={props.className}
                name={props.name}
                placeholder={props.placeholder}
                type={props.type}
                value={props.value}
                onChange={props.onChange}
            />
        </>
    )
}

export default Input