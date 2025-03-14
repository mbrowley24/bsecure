import { useCallback, useState } from "react";
import axios from "axios";
import { useNavigate } from "react-router";

const useHttp = () =>{
    const navigate = useNavigate();
    const [isLoading, setIsLoading] = useState(false);

    const httpRequest = useCallback((requestConfig, applyData)=>{
        // console.log(requestConfig)
            setIsLoading(true);
            axios({
                method: requestConfig.method? requestConfig.method : 'GET',
                baseURL: "http://localhost:8080/api/v1/",
                url: requestConfig.url,
                data: requestConfig.data,
                responseType: requestConfig.responseType? requestConfig.responseType : 'json',
                withCredentials: true,
                headers:{
                    "Content-Type": 'application/json'
                    
                },
                signal : requestConfig.signal
            }).then((response) => {

                if(response.status === 200){
                    applyData(response);
                    setIsLoading(false);
                }
            }).catch((error)=>{
                console.log(error)
                if(error.response.status === 403){

                    navigate('/login')
                }else{

                    applyData(error);
                }

            })

    }, [navigate])


    return(
        {
            httpRequest,
            isLoading,
            setIsLoading,
        }
    )

}

export default useHttp;