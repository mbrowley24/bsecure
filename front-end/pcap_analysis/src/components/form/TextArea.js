import {} from 'react';




const TextArea = (props) =>{


    return (
        <>
            <textarea
                      className={props.className}
                      cols={props.cols}
                      name={props.name}
                      rows={props.rows}
                      value={props.value}
                      onChange={props.onChange}
            >

            </textarea>
        </>
    )
}

export default TextArea;