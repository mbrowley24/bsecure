import {useState} from "react";
import {useNavigate} from "react-router-dom";
import useHttp from "../../hooks/useHttp";
import Input from "../../components/form/Input.js";
import Button from "../../components/form/Button.js";
import '../../css/login.css'

function FormLogin(){
    const navigate = useNavigate();
    const [login, setLogin] = useState({
        username: "",
        password: ""
    });
    const {httpRequest} = useHttp();


    function handleSubmit(e){
        e.preventDefault();

        const configRequest={
            method: "POST",
            url: "login",
            data: login
        }

        function applyData(res){

            if(res.status === 200){
                console.log(res);
                navigate('/dashboard');    
                
            }else if (res.status === 403){
            
                alert("Invalid username or password");
            
            }
        }

        httpRequest(configRequest, applyData);
    }

        


    return(
        <div className="login_container">
            <div className="login">
                <form  onSubmit={handleSubmit}>
                    <div>
                        <Input
                            className="input"
                            placeholder="Username"
                            type="text"
                            value={login.username}
                            onChange={(e)=>setLogin({...login, username: e.target.value})}
                            label="Username"
                        />
                    </div>
                    <div>
                        <Input
                            className="input"
                            placeholder="Password"
                            type="password"
                            value={login.password}
                            onChange={(e)=>setLogin({...login, password: e.target.value})}
                            label="Password"
                        />
                    </div>
                    <div className="submit">
                        <Button
                            className="button"
                            label="Login"
                        />
                    </div>
                </form>
            </div>
        </div>
    )
}

export default FormLogin;