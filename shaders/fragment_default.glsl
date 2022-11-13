/*
#version 140
in vec3 vColor;
out vec4 f_color;

#define MAX_DIST 100
#define MAX_STEPS 100
#define MIN_DIST 0.001

float dist_sphere(vec3 p){
	vec3 pos = vec3(0, 0, 3);
	float r = 0.20;
	return min(length(pos - p) - r, 0.2);
}

float rayMarch(vec3 ro, vec3 rd) {
	float d0 = 0.0;

	for(int i = 0; i < MAX_STEPS; i++) {
		vec3 p = ro + rd*d0;
		float dS = dist_sphere(p);
		d0 += dS;
		if(d0 > MAX_DIST || dS < MIN_DIST) break;
	}

	return d0;
}

void main() {
	vec2 res = vec2(800.0, 600.0);
	vec2 uv = (gl_FragCoord.xy-0.5*res.xy)/res.y;
	vec3 p = vec3(0, 1, 0);
	vec3 rd = vec3(0, 0, 1);
	float rm = rayMarch(uv.xyy, rd);
	vec3 color = vec3(rm/200);

	f_color = vec4(vColor, 1.0);
}
*/

#version 140

#define MAX_STEPS 255
#define MIN_DIST 0.0
#define MAX_DIST 100.0
#define EPSILON  0.001


uniform float iTime;
uniform vec2 iResolution;

in vec3 vColor;



float sdPlane(vec3 p, float h) {
    return p.y - h;
}

float sdSphere(vec3 p) {
    return length(p) - 1.0;
}

float sdBox( vec3 p, vec3 b )
{
  vec3 q = abs(p) - b;
  return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}

float sdTorus( vec3 p, vec2 t )
{
  vec2 q = vec2(length(p.xz)-t.x,p.y);
  return length(q)-t.y;
}

float opSmoothUnion( float d1, float d2, float k ) {
    float h = clamp( 0.5 + 0.5*(d2-d1)/k, 0.0, 1.0 );
    return mix( d2, d1, h ) - k*h*(1.0-h); }

float sdScene(vec3 p) {

    float plane = sdPlane(p, -1.0);
    float box = sdBox(p, vec3(0.8));
    float torus = sdTorus(p, vec2(1.0, 0.1));
    
    float mix = opSmoothUnion(box, torus, 0.1);

    return min(plane, mix);

}


float rayMarching(vec3 eye, vec3 dir, float start, float end){
    float depth = start;
    
    for(int i = 0; i < MAX_STEPS; i++) {
        float dist = sdScene(eye + depth * dir);
        
        if(dist < EPSILON) {
            return depth;
        }
        
        depth += dist;
        
        if(depth >= end){
            return end;
        }
    }
    
    return end;
}

vec3 rayDirection(float fov, vec2 size, vec2 fragCoord) {
    vec2 xy = fragCoord - size / 2.0;
    float z = size.y / tan(radians(fov) / 2.0);
    return normalize(vec3(xy, -z));
}

vec3 estimateNormal(vec3 p) {
    return normalize(vec3(
        sdScene(vec3(p.x + EPSILON, p.y, p.z)) - sdScene(vec3(p.x - EPSILON, p.y, p.z)),
        sdScene(vec3(p.x, p.y + EPSILON, p.z)) - sdScene(vec3(p.x, p.y - EPSILON, p.z)),
        sdScene(vec3(p.x, p.y, p.z + EPSILON)) - sdScene(vec3(p.x, p.y, p.z - EPSILON))
    ));
        
}

float light(vec3 p, vec3 lightPos){
    vec3 normal = estimateNormal(p);
    vec3 lightDir = normalize(lightPos - p);
    
    float diff = max(dot(normal, lightDir), 0.0);
    
    return diff;
}

float shadow(vec3 p, vec3 lightPos) {
    vec3 normal = estimateNormal(p);
    vec3 lightDir = normalize(lightPos - p);
    vec3 newPoint = p + normal*EPSILON;
    
    float k = 32.0;
    float depth = 0.0;
    float ress = 1.0;
    
    for(int i = 0; i < MAX_STEPS; i++) {
        float dist = sdScene(newPoint + depth * lightDir);
        
        if(dist < EPSILON) {
            return 0.0;
        }
        ress = min( ress, k*dist/depth );
        depth += dist;
        
        if(depth >= MAX_DIST){
            return ress;
        }
    }
      
    
    return ress;
    
}

mat4 viewMatrix(vec3 eye, vec3 center, vec3 up) {
	vec3 f = normalize(center - eye);
	vec3 s = normalize(cross(f, up));
	vec3 u = cross(s, f);
	return mat4(
		vec4(s, 0.0),
		vec4(u, 0.0),
		vec4(-f, 0.0),
		vec4(0.0, 0.0, 0.0, 1)
	);
}



void main()
{

    vec3 dir = rayDirection(45.0, iResolution.xy, gl_FragCoord.xy);
    vec3 eye = vec3(-5.0, 4.0 + sin(iTime*1.3)*0.5, 8.0);
    vec3 center = vec3(0.0, 0.0, 0.0);
    
    mat4 viewMat = viewMatrix(eye, center, vec3(0.0, 1.0, 0.0));
    vec3 lightDir = (viewMat * vec4(dir, 0.0)).xyz;
    
    float dist = rayMarching(eye, lightDir, MIN_DIST, MAX_DIST);
    
    
    vec3 lightPos  = vec3(sin(iTime)*10.0, 8.0, cos(iTime)*10.0);
    
    if (dist > MAX_DIST - EPSILON) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
        //return;
    }

    // Output to screen
    vec3 p = eye + lightDir*dist;
    float diff = light(p, lightPos );
    float shad = shadow(p, lightPos );
    gl_FragColor = vec4(vec3(1.0)*diff*shad,1.0);
}
