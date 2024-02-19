import { useState, React } from "react";
import "./App.css";
import { InputNumber } from 'primereact/inputnumber';
import { Button } from 'primereact/button';
import 'katex/dist/katex.min.css'
import { InlineMath, BlockMath } from 'react-katex';
import { InputTextarea } from 'primereact/inputtextarea';
import katex from 'katex';
import Papa from "papaparse";

const N = 20;
let no = 1;

function ParseMat(data) {
    if (data == null) {
        return null;
    }
    const lines = data.trim().split('\n');
    const matrix = lines.map(line =>
        line.split(' ').map(value => parseFloat(value))
    );
    return matrix;
}
function ParseFile(data) {
    return new Promise((resolve, reject) => {
        Papa.parse(data, {
            complete: function (results) {
                try {
                    if (!results.data || results.data.length === 0) {
                        throw new Error('No data available in file');
                    }
                    let result = {
                        N: 0,
                        eps: 0,
                        A: [],
                        b: []
                    };
                    const headers = results.data[0];
                    result.N = parseInt(headers[0], 10);
                    result.eps = parseFloat(headers[1]);

                    for (let i = 0; i < result.N; i++) {
                        result.A.push([]);
                        for (let j = 0; j < result.N; j++) {
                            result.A[i].push(parseFloat(results.data[i + 1][j]));
                        }
                    }

                    for (let j = 0; j < result.N; j++) {
                        result.b.push(parseFloat(results.data[result.N + 1][j]));
                    }

                    resolve(result);
                } catch (error) {
                    reject(error);
                }
            },
            error: function (error) {
                reject(error);
            }
        });
    });
}

function Run(n, eps, A, b, file) {
    console.log(file);
    let runCalculation = ({ N, eps, A, b }) => {
        console.log(A, b, N, eps);
        validate(A, b, N);

        console.log(JSON.stringify({ A, b, n: N, eps }));

        const requestOptions = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': '*'
            },
            body: JSON.stringify({ A, b, n: N, eps })
        };

        return fetch('http://127.0.0.1:6379/', requestOptions)
            .then(response => response.json())
            .then(data => setRow(N, eps, data.x, data.acc, data.iters, data.error))
            .catch(error => console.error('Error:', error));
    };

    if (file != null) {
        return ParseFile(file).then(runCalculation).catch(error => console.error('File parsing error:', error));
    } else {
        A = ParseMat(A);
        b = ParseVec(b);
        return runCalculation({ N: n, eps, A, b });
    }
}

function ParseVec(data) {
    if (data == null) {
        return null;
    }
    const lines = data.trim().split(' ');
    const vec = lines.map(value => parseFloat(value));
    return vec;
}

function validate(A, b, n) {
    if (A === null || b === null) {
        alert('Matrix A and vector b cannot be null.');
        throw new Error('Matrix A and vector b cannot be null.');
    }

    if (A.length !== n || b.length !== n) {
        alert(`Both matrix A and vector b should have dimensions equal to N (${n}).`);
        throw new Error(`Both matrix A and vector b should have dimensions equal to N (${n}).`);
    }

    const isMatrixValid = A.every(row => row.length === n && row.every(Number.isFinite));
    const isVectorValid = b.every(Number.isFinite);

    if (!isMatrixValid || !isVectorValid) {
        alert('Matrix A and vector b must consist of floats.');
        throw new Error('Matrix A and vector b must consist of floats.');
    }

    return true;
}

function setRow(N, eps, x, L, it, errors) {
    console.log(N, eps, x, L, it, errors);

    let formatted_x = "(";
    for (let i = 0; i < x.length; i++) {
        formatted_x += x[i];
        if (i < x.length - 1) {
            formatted_x += ", ";
        }
    }
    formatted_x += ")";
    let formatted_L = "(";
    for (let i = 0; i < L.length; i++) {
        formatted_L += L[i];
        if (i < L.length - 1) {
            formatted_L += ", ";
        }
    }
    formatted_L += ")";

    let non = katex.renderToString(no.toString());
    let Nn = katex.renderToString(N.toString());
    let epsn = katex.renderToString(eps.toString());
    let itn = katex.renderToString(it.toString());
    let Ln = katex.renderToString(formatted_L);
    let xn = katex.renderToString(formatted_x);

    let ans = `<tr><td>${non}</td><td>${Nn}</td><td>${epsn}</td><td>${xn}</td><td>${Ln}</td><td>${itn}</td><td>${errors}</td></tr>`;
    document.getElementById("j-tbody").innerHTML += ans;
    // console.log(ans);
    no++;

    return (
        <tr><td>{non}</td><td>{Nn}</td><td>{epsn}</td><td>{xn}</td><td>{Ln}</td><td>{itn}</td><td>{errors}</td></tr>
    );
}

function App() {
    let [n, setN] = useState(0);
    let [eps, setEps] = useState(1.0);
    let [mat, setMat] = useState("");
    let [res, setRes] = useState("");
    let [file, setFile] = useState(null);
    let [table, setTable] = useState([]);
    const [parsedData, setParsedData] = useState([]);

    return (
        <div className="container">
            <div className="choose">
                <InputNumber name="choose-N" placeholder="choose N" min={0} max={N} onChange={(e) => setN(e.value)} /><br />
                <InputNumber name="choose-eps" placeholder="choose precision" min={0} minFractionDigits={0} maxFractionDigits={5} onChange={(e) => setEps(e.value)} /><br />
                <input name="choose-from-file" id="kekkekkek" placeholder="load from file" type="file" accept=".csv" onChange={(e) => {
                    setFile(e.target.files[0]);
                }} /><Button onClick={() => {
                    setFile(null);
                    document.getElementById("kekkekkek").value = null;
                }} style={{ 'position': 'absolute', 'margin-top': '10px' }}>Clear</Button><br />
                <InputTextarea name="choose-mat" placeholder="enter coefficients" onChange={(e) => setMat(e.target.value)}></InputTextarea><br />
                <InputTextarea name="choose-res" placeholder="enter resulting vector" onChange={(e) => setRes(e.target.value)}></InputTextarea><br />
                <Button onClick={() => {
                    console.log(Run(n, eps, mat, res, file));
                }}>Run</Button>
            </div>
            <div className="journal">
                <table id="j-table">
                    <thead id="j-thead">
                        <tr id="j-tr">
                            <th id="j-no"><InlineMath math="№" /></th>
                            <th id="j-N"><InlineMath math="N" /></th>
                            <th id="j-eps"><InlineMath math="\varepsilon" /></th>
                            <th id="j-ans"><InlineMath math="x" /></th>
                            <th id="j-acc"><InlineMath math="\mathcal{L}^1" /></th>
                            <th id="j-iter">iterations</th>
                            <th id="j-err">errors</th>
                        </tr>
                    </thead>
                    <tbody id='j-tbody'>
                        {table}
                    </tbody>
                </table>
            </div>
        </div >
    );
}

export default App;
