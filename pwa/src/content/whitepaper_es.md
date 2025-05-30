# Whitepaper de Asami Club

Este whitepaper describe el sistema tal como está concebido. Algunas funciones ya existen, otras están en desarrollo. { .lead }

---

## 1. Resumen

Asami.club es un protocolo descentralizado que permite a cualquiera financiar o ganar dinero con la creación de tendencias en redes sociales. En el centro del sistema hay un mecanismo simple pero poderoso: un Anunciante reserva un presupuesto denominado en USD para amplificar un mensaje, y los Colaboradores ganan una parte de ese presupuesto al republicarlo en sus propias cuentas de X (anteriormente Twitter).

Este presupuesto se mantiene y paga en DOC (Dollar on Chain), una stablecoin respaldada por Bitcoin, lo que garantiza pagos confiables y no expuestos a la volatilidad del mercado cripto. DOC es la principal recompensa económica que reciben los Colaboradores: es cómo se les paga por su trabajo, de forma directa y en tiempo real.

Junto con los pagos en DOC, todos los participantes también reciben tokens ASAMI. Estos tokens cumplen múltiples funciones: actúan como señal de alineación con el club, otorgan derechos de gobernanza sobre la tasa de comisión del protocolo, y permiten participar del reparto de ingresos generados por las comisiones. Este sistema dual de recompensas ofrece incentivos tanto inmediatos (DOC) como a largo plazo (ASAMI).

El protocolo está implementado como un contrato inteligente en Rootstock (RSK), una cadena lateral de Bitcoin que soporta contratos inteligentes compatibles con Ethereum. Rootstock fue elegido por su modelo de seguridad (minado combinado con Bitcoin), su historial de disponibilidad y su uso de Bitcoin como moneda nativa. Todas las recompensas y presupuestos de las campañas están en DOC, mientras que los tokens ASAMI se distribuyen en base a las comisiones retenidas.

---

## 2. Historia del proyecto, descubrimientos y estado actual

Asami.club comenzó como una idea experimental basada en las capacidades emergentes de las redes sociales descentralizadas y la infraestructura web3. En las primeras exploraciones, nos dimos cuenta de que, aunque las plataformas sociales siempre se han monetizado mediante publicidad, el trabajo real de difundir influencia —dar likes, republicar, comentar— casi nunca fue compensado de manera justa o directa.

### 2.1 De Nostr a X

Nuestro primer prototipo se construyó sobre Nostr, un protocolo de redes sociales descentralizado. Desarrollamos un contrato inteligente que aceptaba fondos de Anunciantes y permitía a cualquiera reclamar una porción de esos fondos publicando un mensaje verificable en Nostr. Estas reclamaciones se aseguraban mediante firmas criptográficas nativas del protocolo. Si bien el modelo era conceptualmente elegante y completamente descentralizado, presentaba dos problemas principales:

- Las operaciones criptográficas eran costosas, lo que generaba altas comisiones en cadena.
- La base de usuarios de Nostr en ese momento era demasiado pequeña para atraer interés significativo de los Anunciantes.

Entonces pivotamos hacia X (anteriormente Twitter), reconociendo que era necesario un puente entre los medios tradicionales y los descentralizados. Esto requirió un modelo híbrido donde las colaboraciones se verifican fuera de la cadena y son enviadas por un oráculo: específicamente, el Administrador de campañas.

### 2.2 Evaluando las suposiciones del mercado

La idea central de Asami se basaba en dos suposiciones:

- Que **los usuarios de redes sociales estarían dispuestos a que se les pague por republicar contenido**, y
- Que **las marcas y proyectos verían valor en las republicaciones realizadas por personas reales**, especialmente en comparación con el engagement falso comprado por medios tradicionales.

Confirmamos que existían ambos lados del mercado. Los Colaboradores se sumaron con entusiasmo y estaban dispuestos a republicar contenido a cambio de recompensas. Los Anunciantes, por su parte, valoraron la idea de una amplificación real y dirigida por pares. Sin embargo, surgieron desafíos al transformar ese potencial en un mercado funcional y eficiente.

Los pagos basados en cripto fueron una **súper herramienta del lado de los Colaboradores**: permitían recompensas instantáneas, sin fronteras y sin intermediarios. Pero también **limitaban la base de Anunciantes**, ya que muchas marcas todavía dependen de la infraestructura fiat tradicional. Asami necesitará integrar rampas fiat como Stripe para abordar completamente esta desalineación.

Y como en cualquier mercado, **el descubrimiento de precios** es clave. Confirmamos que había oferta y demanda, pero los dos lados tenían dificultades para coincidir en el precio justo. Muchos Colaboradores esperaban pagos altos (por ejemplo, USD 5 por republicación) sin importar el tamaño o nivel de interacción de su audiencia. Los Anunciantes, en cambio, descubrieron que las campañas no entregaban suficiente valor por dólar invertido. Las campañas empezaron a ofrecer recompensas más bajas, lo que alejó a los Colaboradores valiosos y dejó a aquellos que estaban sobrepagados, reduciendo aún más la efectividad de las campañas.

### 2.3 Fricción de puntuación e insuficiencia de Colaboradores

Este espiral descendente reveló dos problemas clave:

1. **Algoritmos defectuosos de medición de influencia**, que resultaban en sobrepagos a cuentas de baja calidad, permitiendo abusos y reduciendo la satisfacción de los Anunciantes. Respondimos diseñando un nuevo algoritmo (descrito en este whitepaper) que introduce mediciones de autoridad más estrictas y en capas.

2. **Un número insuficiente de Colaboradores** que llevaba a rigidez en los precios. Un pequeño grupo de participantes podía dominar las recompensas de las campañas y desincentivar el crecimiento. Peor aún, algunos estaban desincentivados a invitar a otros, ya que eso diluía sus ganancias.

Para contrarrestar esto, introdujimos un mecanismo de recompensa dedicado a las referencias exitosas. Ahora, invitar a nuevos miembros valiosos está incentivado directamente y se reconoce dentro del puntaje de influencia.

### 2.4 Lecciones sobre funciones y soluciones futuras

Surgieron ideas adicionales para alinear mejor los incentivos y ayudar a que el mercado funcione:

- **Retroalimentación de precios impulsada por Colaboradores**: los Colaboradores deberían poder sugerir su tarifa preferida al aceptar o rechazar una campaña. Estos datos podrían ayudar a los Anunciantes a fijar mejor los precios y ajustar expectativas.

- **Listas de permitidos/bloqueados para Anunciantes**: los Anunciantes necesitan mejores herramientas para seleccionar quién puede participar en sus campañas. Estas listas les permitirán bloquear a Colaboradores poco efectivos o fuera de marca, y brindarán señales para identificar fatiga de influencia.

### 2.5 Lo que aprendimos y lo que viene

Tras un año completo de funcionamiento del protocolo en el mundo real, dos conclusiones son innegables:

1. **Necesitamos más Colaboradores.** La plataforma solo funciona si los Anunciantes pueden acceder a una amplia variedad de personas reales que amplifiquen sus mensajes. El crecimiento orgánico no es suficiente: debemos atraer activamente nuevos Colaboradores. Esto probablemente requiera esfuerzos dedicados de marketing y publicidad paga.

2. **Necesitamos un sistema de medición de influencia confiable.** Errores en el algoritmo de puntuación pueden desestabilizar toda la economía, recompensando a los actores equivocados y socavando la confianza de los Anunciantes. El algoritmo debe mantenerse cuidadosamente, estar abierto a revisiones y validarse de manera continua.

Ambas responsabilidades —hacer crecer la base de Colaboradores y mantener la lógica de puntuación— recaen naturalmente en los **Administradores de campañas**, quienes están en el centro de la relación entre Anunciantes, Colaboradores y el protocolo. A día de hoy, solo hay un Administrador de campañas cumpliendo este rol, y está trabajando activamente en soluciones.

---

## 3. Roles del ecosistema y participantes

Esta sección describe a los participantes clave en el ecosistema de Asami y los roles que desempeñan dentro del protocolo. Cada tipo de parte interesada —Anunciantes, Colaboradores, Administradores de campañas y Titulares de tokens— tiene una función específica, un conjunto de incentivos, derechos y responsabilidades. Comprender estos roles es esencial para entender cómo funciona el sistema Asami y cómo fluyen las decisiones, el valor y las recompensas a través de la red.

### 3.1 Anunciantes

**Quiénes son:** Cualquier persona que quiera promocionar una publicación. Esto puede incluir individuos, proyectos, startups, agencias, marcas o fans que apoyan a otra persona.

**Qué hacen:** Un Anunciante crea una “campaña” seleccionando una publicación (generalmente de X), definiendo un presupuesto total en DOC, estableciendo una duración y especificando cómo se distribuyen las recompensas entre los Colaboradores según sus puntajes de influencia.

**Derechos:**
- Decidir cuánto pagar por unidad de influencia.
- Establecer pagos mínimos y máximos para asegurar una distribución amplia del presupuesto.
- Agregar más fondos o extender una campaña en cualquier momento.
- Retirar los fondos no utilizados una vez finalizada la campaña.
- Elegir un Administrador de campañas para cada campaña y cambiar libremente de uno a otro.
- Decidir qué cuentas están permitidas o bloqueadas para participar en su campaña, utilizando listas de permitidos/rechazados con la ayuda del Administrador de campañas.

**Obligaciones:**
- Aceptar que una vez lanzada una campaña, no se puede cancelar.
- Confiar en que el Administrador de campañas registrará de forma justa las republicaciones y aplicará el método de puntuación de influencia.

**Notas adicionales:**
Los Anunciantes no tienen garantizado que los Colaboradores republicarán su mensaje. Los Colaboradores eligen campañas voluntariamente, creando un sistema de autoselección en el que solo los usuarios alineados amplifican un mensaje.

### 3.2 Colaboradores

**Quiénes son:** Usuarios de redes sociales con audiencias reales que eligen amplificar mensajes y ganar dinero por hacerlo.

**Qué hacen:** Los Colaboradores exploran la lista de campañas activas para las que son elegibles y deciden qué mensajes desean republicar. Cuando republican la publicación de una campaña, su acción es registrada por el Administrador de campañas y reciben un pago en DOC.

**Derechos:**
- Elegir qué campañas republicar. Nadie está obligado a participar.
- Recibir pagos en DOC por republicaciones exitosas, según su nivel de influencia y las reglas de pago de la campaña.
- Recibir tokens ASAMI como parte del reparto de ingresos del protocolo.
- Solicitar una revisión de su puntaje de influencia o asignación de categoría (por ejemplo, idioma), siguiendo un proceso definido.

**Obligaciones:**
- Mantener las republicaciones por un período mínimo. Eliminaciones prematuras pueden afectar su elegibilidad futura.
- Operar una cuenta legítima en X con actividad genuina. Las cuentas que solo republican sin publicaciones originales, o muestran señales de interacción artificial, pueden ser excluidas.

**Limitaciones y detalles importantes:**
- Las colaboraciones se registran por orden de llegada. Una vez que una campaña se queda sin fondos, no se realizan más pagos.
- Si una republicación es válida pero no tiene fondos, puede marcarse como “no pagada” y aún así ganar tokens ASAMI, o ser elegible para recompensas futuras si se agregan fondos más adelante.
- Los Colaboradores pueden ser filtrados de ciertas campañas debido a puntajes bajos, categorías faltantes u otros factores establecidos por los Anunciantes o el Administrador de campañas.

**Disputas sobre puntajes de influencia:**
Si un Colaborador cree que su puntaje de influencia es incorrecto, debe:
- Leer la sección de Medición de Influencia de este whitepaper.
- Identificar la parte exacta del algoritmo que considera mal aplicada.
- Comparar su puntaje con el de cuentas similares.
- Proporcionar evidencia (por ejemplo, métricas de interacción, resultados de encuestas, estadísticas de referidos).

Quejas vagas como “mi puntaje es muy bajo” serán redirigidas a este proceso. No se iniciará ninguna revisión sin una solicitud clara basada en evidencia.

### 3.3 Administradores de campañas

**Quiénes son:** Operadores que gestionan campañas, calculan la influencia, registran republicaciones y, opcionalmente, brindan servicios adicionales a las otras partes.

**Qué hacen:** El Administrador de campañas es el actor principal que conecta a Anunciantes y Colaboradores. Registra las republicaciones, ejecuta el algoritmo de puntuación de influencia, aplica las reglas y gestiona los pagos (incluidos los retiros sin gas).

**Derechos:**
- Implementar y modificar sus propios algoritmos de medición de influencia.
- Determinar qué Colaboradores son elegibles para cada campaña.
- Registrar las republicaciones enviadas por Colaboradores elegibles y asignar recompensas en DOC de acuerdo con la estructura de pagos basada en influencia.
- Mantener listas de permitidos/rechazados de usuarios, asignar categorías (por ejemplo, idioma o región) y validar la interacción.
- Establecer una tarifa opcional por su servicio, además de la tarifa del protocolo.
- Ofrecer servicios adicionales (por ejemplo, redacción de contenido, estrategia) a Anunciantes y Colaboradores.

**Obligaciones:**
- Actuar dentro de los límites de las reglas de cada campaña del Anunciante.
- Utilizar métodos de medición de influencia claros, reproducibles, y divulgar su lógica.
- Responder a las consultas de los Colaboradores, especialmente en cuestiones relacionadas con puntajes o elegibilidad.

**Limitaciones:**
- Una vez que una colaboración es registrada y los fondos pagados, la acción es definitiva. No pueden recuperarse recompensas, incluso si luego se detecta un abuso.
- Los Administradores de campañas no tienen control sobre el contrato inteligente de Asami.
- Si sus decisiones se consideran injustas o poco claras, pueden perder la confianza de Anunciantes y Colaboradores, lo que puede dar lugar a que surjan otros Administradores de campañas alternativos.

### 3.4 Titulares de tokens

**Quiénes son:** Cualquier persona que posea tokens ASAMI. Esto incluye a todos los Colaboradores, Anunciantes y Administradores de campañas que hayan ganado tokens.

**Qué hacen:** Los titulares de tokens ASAMI participan en la dirección a largo plazo del protocolo al recibir ingresos y votar sobre la tasa de comisión.

**Derechos:**
- Recibir una parte proporcional de las comisiones retenidas por el protocolo.
- Votar sobre cambios en la tasa de comisión del protocolo usando un sistema de promedio ponderado.
- Mantener, transferir o vender libremente sus tokens.

**Obligaciones:**
- Ninguna impuesta por el protocolo, pero se espera una votación responsable y alineación con el valor a largo plazo.

**Notas adicionales:**
- Los titulares de tokens no tienen influencia directa sobre las campañas ni sobre los roles.
- El único poder de gobernanza disponible actualmente es la capacidad de votar sobre la tasa de comisión del protocolo. Consulta la sección de Tokenomics para más detalles sobre cómo funciona la emisión y gobernanza de ASAMI.

---

## 4. Medición de influencia

La influencia dentro de Asami se calcula usando un modelo estructurado que la define como el producto del **tamaño de audiencia** y la **autoridad**. Este modelo asegura equidad y escalabilidad, al mismo tiempo que ofrece a los Administradores de campañas flexibilidad para adaptar los métodos de puntuación.

### 4.1 Tamaño de audiencia

El tamaño de audiencia es una medida cuantitativa de cuántas personas realmente ven las publicaciones de un usuario. Inicialmente, se utilizaban los conteos de seguidores, pero el sistema actual se basa en la cantidad de impresiones de tweets en los últimos 45 días. Esto brinda una visión más precisa y en tiempo real del alcance real.

Esta medición es pesimista: no se asume ninguna audiencia a menos que pueda demostrarse mediante impresiones. Las impresiones aún pueden ser manipuladas, por lo que el sistema verifica la correlación estadística entre impresiones e interacciones (me gusta, comentarios, republicaciones). Las cuentas con proporciones anormales o signos de manipulación pueden tener su tamaño de audiencia ajustado a cero.

### 4.2 Autoridad

La autoridad refleja cuánta influencia real tiene un usuario sobre su audiencia, independientemente del tamaño. Una audiencia pequeña pero altamente leal puede ser más valiosa que una grande e indiferente. El concepto de autoridad es sutil y complejo, y Asami lo aborda mediante un sistema de múltiples criterios. Cada uno de los siguientes criterios contribuye a un porcentaje de autoridad, que va del 0% al 100%. Si no se puede probar autoridad, el puntaje es cero.

Un Administrador de campañas puede aplicar estas métricas de forma automática, subjetiva o mediante revisión manual. Estos son los factores que contribuyen a la puntuación de autoridad y su propósito:

#### Interacción recibida en X
Este es el requisito fundamental. Si las publicaciones de una cuenta no generan interacción de usuarios reales, probablemente no tenga influencia real.
- **Nula**: Sugiere que las publicaciones son ignoradas o la audiencia es falsa. Resultado: 0% de autoridad, se omiten los demás criterios.
- **Media**: Muestra interacción regular. Suma 25%.
- **Alta**: Indica fuerte interés en la cuenta. Suma 50%.

#### Encuestas directas a la audiencia
El usuario puede publicar una encuesta anónima preguntando a sus seguidores cuánto confían en sus recomendaciones. Esto ofrece una percepción sobre cómo lo ve su audiencia.
- **Ninguna**: Sin datos útiles. Suma 0%.
- **Media**: Indica confianza moderada. Suma 10%.
- **Alta**: Demuestra seguidores leales. Suma 20%.
- **Inversa**: La mayoría responde que haría lo contrario de lo que recomienda el usuario. Esto **divide por la mitad** el puntaje de “Interacción recibida en X”.

> Ejemplo de encuesta:  
> _“Si recomiendo algo, ¿qué sueles hacer? a) Lo sigo ciegamente, b) Lo considero, c) Lo ignoro, d) Hago lo contrario.”_

#### Interacción fuera de X
Algunos individuos son influyentes en otros contextos: tienen podcasts, escriben libros, lideran comunidades, etc. Este criterio considera la reputación fuera de la plataforma.
- **Nula**: Suma 0%.
- **Media**: Presencia pública verificada. Suma 5%.
- **Alta**: Figura reconocida y respetada. Suma 10%.

#### Estado de la cuenta en X
El estado operativo de la cuenta afecta su credibilidad.
- **Suspendida/Shadowbanned**: Autoridad se ajusta a 0% y se omiten los demás criterios.
- **Normal**: Sin cambios.
- **Mejorada**: Cuenta verificada, premium o de confianza. Suma 10%.

#### Autoridad por referidos
Usuarios que invitan con éxito a otros a unirse a Asami demuestran influencia, especialmente si esos referidos son de calidad.
- **Referidos válidos desde cuentas activas**: Suma 10%.

#### Comportamiento de tenencia de tokens
Dado que los Anunciantes y Administradores de campañas reciben pagos en ASAMI, pueden preferir recompensar a quienes están alineados con el éxito del club.
- **Mantener tokens ASAMI en vez de venderlos**: Suma 10%.

### 4.3 Cálculo del puntaje de autoridad

El porcentaje final de autoridad se calcula de la siguiente forma:
- Comienza con la **Interacción en X**. Si es Nula, el puntaje total es 0%.
- Si es Media o Alta, se suma el valor base (25% o 50%), luego se aplica:
  - **Encuestas a la audiencia**: Suma 0%, 10%, 20%, o divide el puntaje en caso de respuesta Inversa.
  - **Interacción fuera de X**: Suma 0%, 5% o 10%.
  - **Estado en X**: Suma 0% o 10%, excepto si está suspendida, en cuyo caso el puntaje es 0%.
  - **Referidos**: Suma 10% si cumple criterios.
  - **Tenencia de tokens**: Suma 10% si corresponde.

El puntaje de autoridad se multiplica luego por el tamaño de audiencia para calcular el **Nivel de influencia**, que se usa en la distribución de recompensas.

Este sistema en capas busca ser justo, transparente y resistente a manipulaciones, a la vez que permite evolucionar conforme surjan nuevas señales relevantes.

---

## 5. Tokenomics y distribución de ASAMI

El token ASAMI es el activo nativo del protocolo Asami.club. Tiene un suministro máximo fijo de **21 millones de tokens**, similar al de Bitcoin, y está diseñado para distribuir la propiedad de los ingresos y el crecimiento del protocolo.

### 5.1 Reparto de ingresos e incentivos

Cada vez que un Colaborador recibe un pago en DOC por republicar una campaña, el protocolo **retiene un 10%** de ese pago como comisión. Estas comisiones, recolectadas en DOC, se acumulan y se distribuyen cada 15 días entre todos los titulares de tokens ASAMI. El monto que recibe cada titular es proporcional a la cantidad de tokens que posee.

> 📥 Por ejemplo, si tienes el 1% de todos los tokens ASAMI, recibirás el 1% del fondo de ingresos recolectado en ese período.

Esto hace que mantener tokens ASAMI sea atractivo como forma de **ingreso pasivo**, ya que otorga acceso a los ingresos futuros generados por campañas en curso.

> 💡 Mantener ASAMI es como tener una participación en una economía publicitaria descentralizada. Cuantas más campañas se ejecuten a través del protocolo, más grande será el fondo de ingresos.

Esta estructura incentiva a todos los participantes a hacer crecer la plataforma:
- **Los Anunciantes** financian campañas y reciben tokens ASAMI como parte del pago de cada campaña.
- **Los Colaboradores** ganan ASAMI además de sus recompensas en DOC.
- **Los Administradores de campañas** reciben la mayor proporción de tokens ASAMI cuando registran colaboraciones activamente.

### 5.2 Gobernanza mediante votación de comisión

Los titulares de ASAMI también pueden votar sobre la tasa de comisión del protocolo.

El sistema de votación utiliza un **modelo de promedio ponderado**: cada titular de tokens propone un porcentaje que considera adecuado como comisión (por ejemplo, 5%, 15%, 30%). El valor final se calcula promediando todos los votos, ponderados por la cantidad de tokens que posee cada votante.

> 🗳️ Por ejemplo, si un usuario con 10.000 tokens vota por un 5% y otro con 1.000 tokens vota por un 100%, el resultado reflejará el promedio ponderado, no la mediana.

Este diseño de “tira y afloja” genera dinámicas equilibradas:
- **Comisiones más altas** generan más reparto de ingresos, pero también mayor inflación de tokens.
- **Comisiones más bajas** reducen las ganancias de los titulares, pero pueden atraer más Anunciantes y Colaboradores.

Esta herramienta de gobernanza permite a los titulares de tokens influir en la economía a largo plazo del protocolo, equilibrando sus propios incentivos.

### 5.3 Modelo de emisión y equidad

El protocolo tiene como objetivo emitir **100.000 tokens ASAMI cada 15 días**, pero esta cantidad se ajusta dinámicamente según:
- Las comisiones en DOC recolectadas durante el período anterior.
- El total de tokens ASAMI emitidos hasta la fecha.

Los tokens se emiten cada vez que se paga una recompensa de campaña. La comisión retenida por el protocolo (10% del pago en DOC) se convierte en tokens ASAMI usando la tasa de emisión vigente. Estos tokens se reparten entre los participantes de la siguiente manera:
- **40% para el Administrador de campañas**
- **30% para el Colaborador**
- **30% para el Anunciante**

> 🧮 **Ejemplo:**  
> Una campaña paga 20 DOC a un Colaborador. El protocolo retiene 2 DOC (10%).  
> Si la tasa de emisión es de 1000 ASAMI por DOC, se emiten 2000 ASAMI:  
> - 800 para el Administrador de campañas  
> - 600 para el Colaborador  
> - 600 para el Anunciante

No hubo **premine**, ni se hizo ninguna **asignación especial** a personas internas. El único gran poseedor de tokens (“ballena”) es el actual Administrador de campañas, que obtuvo aproximadamente el 40% de los tokens emitidos simplemente por ser el único que ha registrado todas las colaboraciones hasta la fecha.

Este sistema de emisión, transparente y justo, garantiza que:
- La participación sea recompensada de forma proporcional.
- La distribución de tokens refleje la actividad real en la plataforma.
- Los incentivos se mantengan alineados para lograr una sostenibilidad a largo plazo.

Combinando reparto de ingresos, poder de voto y una emisión equitativa, los tokens ASAMI funcionan tanto como un activo generador de rendimiento como una herramienta de gobernanza descentralizada.

---

## 6. Arquitectura técnica

El protocolo Asami está implementado como un contrato inteligente de código abierto desplegado en la blockchain de **Rootstock**. Rootstock es una cadena lateral de Bitcoin con compatibilidad total con la EVM, lo que permite a los desarrolladores usar herramientas familiares de Ethereum mientras aprovechan el modelo de seguridad de Bitcoin.

### 6.1 ¿Por qué Rootstock?

Rootstock fue elegida porque ofrece:

- **Minería combinada con Bitcoin**, lo que refuerza su seguridad.
- **Compatibilidad con la EVM**, que permite un despliegue rápido de contratos inteligentes.
- **Estabilidad y confiabilidad**, con producción de bloques constante y sin interrupciones conocidas.
- **Entorno nativo en Bitcoin**, alineado con los objetivos de descentralización de Asami.

Rootstock usa **RBTC** como moneda nativa para el gas y es una red probada con comisiones bajas y operación estable, lo que la convierte en una base sólida para Asami.

### 6.2 Transacciones y pagos de campañas

Los presupuestos de campaña están denominados en **DOC (Dollar on Chain)**, una stablecoin nativa de Rootstock emitida por Money on Chain. DOC es:

- Sobrecolateralizada con Bitcoin
- Ampliamente utilizada en el ecosistema de Rootstock
- Indexada 1:1 con el dólar estadounidense

Los Colaboradores son pagados en DOC, lo que les brinda un mecanismo de pago predecible y confiable, sin exposición a la volatilidad del mercado cripto.

> Si un Colaborador republica un mensaje de campaña y es elegible, recibe un pago en DOC desde el presupuesto de la campaña. El protocolo retiene automáticamente el 10% de ese pago como comisión.

### 6.3 Dirección del contrato inteligente y transparencia

El contrato inteligente que impulsa el protocolo es públicamente verificable y puede consultarse en:  
[Asami Contract on Rootstock Explorer](https://explorer.rootstock.io/address/0x3150e390bc96f1b4a05cf5183b8e7bdb68566954)

El primer Administrador de campañas opera actualmente desde:  
[Campaign Manager Wallet](https://explorer.rootstock.io/address/0x3e79325b61d941e7996f0a1aad4f66a703e24faa)

Todas las interacciones con el protocolo —creación de campañas, registro de colaboraciones y distribución de recompensas— son visibles y trazables en la cadena.

### 6.4 Software del Administrador de campañas

Los Administradores de campañas interactúan con el protocolo utilizando software desarrollado en **Rust**, usando el framework web **Rocket**. Esta aplicación gestiona:

- Detección de colaboraciones
- Puntuación de influencia
- Registro de republicaciones
- Puente entre billeteras Web2 y Web3
- Solicitudes de retiro sin gas (gasless claims)

El código completo es de código abierto y está disponible en:  
[https://github.com/constata-eu/asami](https://github.com/constata-eu/asami)

Esta arquitectura asegura:

- **Modularidad**: otros Administradores de campañas pueden desplegar su propia versión.
- **Transparencia**: el comportamiento y la puntuación pueden ser revisados de forma independiente.
- **Extensibilidad**: nuevas funciones como segmentación por categorías, seguimiento de referidos y encuestas fuera de la cadena son fáciles de implementar.

### 6.5 Retiros sin gas y onboarding de usuarios

Para facilitar la incorporación de usuarios nuevos en Rootstock, el Administrador de campañas ofrece una funcionalidad de **retiro sin gas**. Los Colaboradores:

- Aprueban una deducción de 1 DOC como comisión
- Reciben su pago en DOC y una pequeña cantidad de RBTC para cubrir costos de gas futuros
- Tienen sus recompensas reclamadas en su nombre por el Administrador de campañas

Este mecanismo simplifica el onboarding para personas sin experiencia previa en cripto, y actúa como un tipo de faucet de RBTC, manteniendo la propiedad total de las recompensas.

### 6.6 Integración con pasarelas de pago (futuro)

Para mejorar la experiencia de los Anunciantes, los Administradores de campañas podrán integrar rampas fiat como **Stripe** para:

- Aceptar pagos con tarjeta o transferencia bancaria
- Convertir automáticamente los fondos en DOC
- Financiar campañas directamente desde plataformas Web2

Esto reducirá aún más la barrera de entrada para nuevos Anunciantes.

En resumen, la arquitectura técnica de Asami equilibra descentralización, transparencia y facilidad de uso —impulsada por el entorno seguro y estable de Rootstock, y construida para incorporar a la próxima generación de creadores de tendencias y anunciantes al mundo Web3.

---

## 7. Gobernanza

Asami opera bajo un modelo de gobernanza descentralizado y pragmático. No existen **contratos legales** ni obligaciones fuera de la cadena entre los miembros de la plataforma. Todas las relaciones entre los participantes —Anunciantes, Colaboradores, Administradores de campañas y Titulares de tokens— están gobernadas por la lógica y las restricciones del contrato inteligente de Asami y la discreción práctica de cada parte. La participación es completamente voluntaria y bajo la modalidad “tal como es”.

### 7.1 Los roles se definen por la acción, no por obligación

- **Los Anunciantes** financian campañas, pero no están obligados a incluir a ningún Colaborador específico.
- **Los Colaboradores** eligen en qué campañas participar y pueden rechazar cualquiera sin dar explicaciones.
- **Los Administradores de campañas** actúan a su propia discreción y no están legalmente obligados a actuar en nombre de ninguna parte, más allá del comportamiento que haga cumplir el contrato inteligente.
- **Los Titulares de tokens** pueden participar en la gobernanza, pero no tienen derecho a beneficios directos por parte de los demás participantes.

### 7.2 El modelo “tal como es” y sus consecuencias

Debido a que todas las interacciones son sin permisos y están gobernadas por código:

- No existen **acuerdos de nivel de servicio** entre las partes.
- **Los Anunciantes** no tienen garantía de que un Colaborador participará en su campaña, ni de que el Administrador de campañas actuará de una manera específica más allá de registrar colaboraciones válidas y distribuir recompensas.
- **Los Administradores de campañas** no tienen la obligación legal de reembolsar a los Anunciantes o volver a puntuar a los Colaboradores. Una vez que los fondos son asignados a una campaña, el Administrador de campañas los gestiona de forma autónoma y no es responsable ante los Anunciantes siempre que el proceso siga las reglas del contrato inteligente.
- **Los Colaboradores** no tienen garantizada la participación, visibilidad o pago, a menos que su republicación sea registrada con éxito y esté financiada en una campaña activa. Aun así, el pago no puede ser recuperado en ninguna circunstancia.
- Cualquier **Anunciante** puede excluir a cualquier Colaborador de sus campañas sin dar explicaciones.
- Cualquier **Colaborador** puede rechazar cualquier campaña o abandonar la plataforma en cualquier momento.

### 7.3 Responsabilidad a través de la competencia

Si cualquier parte (Anunciante, Colaborador o Titular de tokens) está insatisfecha con el accionar de un Administrador de campañas:

- Puede dejar de utilizar los servicios de ese Administrador.
- Puede **apoyar o financiar** la creación de otro Administrador de campañas que siga reglas distintas o aplique un algoritmo de puntuación más justo.

El sistema se autorregula mediante elección y transparencia, no mediante coerción o apelaciones. El Administrador de campañas actual no tiene acceso ni privilegios especiales en el protocolo. Cualquier nuevo participante que construya herramientas compatibles puede convertirse en Administrador de campañas y competir por campañas y atención de los Colaboradores.

### 7.4 Votación de parámetros basada en tokens

La única función de gobernanza compartida a nivel de protocolo es la posibilidad de votar sobre la **comisión del protocolo Asami**. Esta comisión está actualmente fijada en el 10% y puede ajustarse mediante un sistema de votación por promedio ponderado:

- Los Titulares de tokens proponen un porcentaje preferido (0–100%) como su voto.
- El valor final se calcula ponderando cada voto según la cantidad de tokens ASAMI que posea cada votante.

> 📊 Ejemplo: Un usuario con 5.000 ASAMI vota por una comisión del 10%. Otro usuario con 500 ASAMI vota por una del 50%. El resultado se acercará más al 10%, basado en el promedio ponderado.

Este mecanismo de votación alinea los incentivos:

- Subir la comisión puede aumentar los ingresos compartidos, pero también causa inflación de tokens y puede desalentar la participación de Colaboradores.
- Bajar la comisión reduce los ingresos compartidos pero puede mejorar el crecimiento de la plataforma y la atracción de campañas.

La simplicidad del protocolo es su fortaleza: incentivos claros, gobernanza mínima y control descentralizado mediante cooperación voluntaria.

---

## 8. Seguridad y prevención de abusos

El protocolo Asami está implementado como un contrato inteligente. Por sí solo, no puede ser abusado, salvo que se descubra un error de software o una vulnerabilidad en su código. Este contrato aplica reglas de forma determinista, gestiona y distribuye fondos, y ejecuta la lógica de emisión de ASAMI, retención de comisiones en DOC y registro en la cadena. Es neutral y agnóstico respecto a la calidad de las campañas o de los actores involucrados.

Por otro lado, los **Administradores de campañas** operan los sistemas que evalúan la elegibilidad, miden los niveles de influencia, registran las republicaciones y distribuyen los fondos de las campañas. Estos sistemas son mucho más susceptibles a manipulaciones y abusos, ya que deben confiar en señales fuera de la cadena y datos provenientes de plataformas como X (Twitter). Dado que los Administradores de campañas deciden cómo se distribuyen los presupuestos entre los Colaboradores, actores maliciosos que simulan ser genuinos pueden intentar **engañar o manipular estos sistemas** para recibir fondos sin aportar valor a los Anunciantes.

Como los Administradores de campañas buscan mantenerse abiertos a la mayor cantidad de Colaboradores posible —fomentando el crecimiento y la participación—, suelen apoyarse en sistemas automatizados para incorporar y evaluar miembros. Esta escalabilidad necesaria también introduce **superficies de ataque** que hacen esencial la detección robusta de abusos.

### 8.1 Tipos de abuso

Las formas más comunes y anticipadas de abuso que enfrentan los sistemas de los Administradores de campañas incluyen:

- **Inflación artificial de impresiones**: Posibles Colaboradores pueden intentar aumentar su visibilidad mediante bots o tráfico comprado. Si las impresiones no son consistentes con métricas de interacción como respuestas o me gusta, el Administrador puede asignar un valor de audiencia igual a cero.

- **Republicaciones sin esfuerzo o a ciegas**: Algunas cuentas republican todas las campañas sin contexto ni interacción genuina. Estas cuentas diluyen el valor de las campañas y son excluidas o despriorizadas.

- **Interacción generada por IA o scripts**: Los Administradores de campañas detectan e ignoran comentarios repetitivos e inconsecuentes (“GM”, “🔥”, etc.) generados por scripts o inteligencia artificial. No se consideran indicadores de influencia real.

- **Influencia inversa**: Cuentas que rutinariamente provocan sentimientos negativos o reacciones adversas pueden ser marcadas mediante encuestas y análisis de sentimiento. Su influencia se ajusta en consecuencia.

- **Influencia fuera de X no capturada por métricas**: Algunas personas influyentes pueden no tener métricas fuertes en X, pero ejercer influencia a través de otros medios (charlas públicas, podcasts, etc.). Estos casos requieren revisión manual de evidencia.

- **Manipulación de referidos / ataques Sybil**: Algunos usuarios pueden intentar ganar autoridad por referidos incorporando cuentas falsas o de baja calidad bajo su control. Los referidos deben ser genuinos y activos para ser contados.

### 8.2 El rol del Administrador de campañas en la prevención

Cada Administrador de campañas implementa su propio algoritmo para puntuar, detectar abusos y determinar la elegibilidad para colaborar. Sus sistemas típicamente incluyen:

- **Puntuación pesimista de audiencia**: No se asigna audiencia a menos que las impresiones estén respaldadas por datos de interacción consistentes.
- **Cálculo de autoridad con múltiples señales**: Interacciones, encuestas, referidos y comportamiento de tenencia de tokens contribuyen al puntaje del Colaborador.
- **Filtrado de elegibilidad**: Solo se aceptan colaboraciones que cumplan con los requisitos de calidad, audiencia y tiempo del Administrador.
- **Etiquetado y bloqueo**: Actores sospechosos de abuso pueden ser despriorizados o excluidos de campañas futuras.

Dado que los Administradores de campañas no están obligados a aceptar a todos los participantes y deben mantener la confianza de los Anunciantes, están facultados para utilizar tanto **lógica automatizada** como **revisión manual**. Sus decisiones no están forzadas por el contrato inteligente, sino que se sostienen por la discreción, la confianza y la competencia. En última instancia, un Administrador que permita abusos perderá la confianza de los Anunciantes.

### 8.3 Derechos y responsabilidades de los Colaboradores

Los Colaboradores que no estén de acuerdo con su puntaje de influencia o asignación de categoría pueden:

- Solicitar una revisión con evidencia específica.
- Comparar su puntaje con el de otros usuarios mediante el tablero público.
- Aportar evidencia externa (por ejemplo, audiencia de un podcast, éxito con referidos).

Sin embargo, deben entender que:

- Los puntajes de influencia son **relativos** y no representan **derechos garantizados**.
- Los Administradores de campañas no están obligados a mantener ni explicar sus decisiones más allá de lo declarado en este whitepaper.
- La participación en campañas queda a discreción del Administrador de campañas.
- Una vez ganadas, las recompensas (en DOC o ASAMI) **no pueden ser recuperadas**.

### 8.4 Transparencia del protocolo y límites

El protocolo Asami solo aplica lógica en la cadena:

- Emite tokens ASAMI
- Recolecta y distribuye comisiones
- Registra pagos en DOC y colaboraciones

No **valida la calidad de las republicaciones ni aplica la medición de influencia**. Estas decisiones suceden fuera de la cadena, en los sistemas de cada Administrador de campañas. Sin embargo, todas las transacciones financieras son transparentes y pueden consultarse públicamente en Rootstock.

Incluso si un Colaborador es bloqueado por un Administrador de campañas, cualquier DOC o ASAMI que haya ganado permanecerá en su billetera y **no puede ser revocado**.

### 8.5 Confianza mediante la competencia

Los Administradores de campañas no tienen un estatus privilegiado dentro del protocolo Asami. Cualquiera puede crear su propio Administrador de campañas, definir sus reglas, construir su comunidad y atender a Anunciantes.

Si los participantes pierden la confianza en un Administrador, pueden:

- Retirarse de sus campañas
- Crear o financiar un nuevo Administrador con estándares más claros
- Apoyar Administradores de campañas con filosofías de puntuación o regiones distintas

El ecosistema de Asami se autorregula. La transparencia, el rendimiento y las alternativas abiertas son los mecanismos que mantienen el abuso bajo control —no la imposición centralizada ni la burocracia.

---

## 9. Crecimiento de la comunidad

El éxito a largo plazo de Asami.club depende del crecimiento sostenido de su comunidad. Cada rol dentro del sistema —Administrador de campañas, Colaborador, Anunciante y Titular de tokens— se beneficia cuando la red crece, se lanzan más campañas y más usuarios participan.

Si bien los incentivos más directos existen para los **Administradores de campañas**, que son recompensados por cada colaboración registrada y ganan la mayor proporción de tokens ASAMI, los demás participantes también tienen razones para contribuir al crecimiento del ecosistema:

- **Los Anunciantes** pueden encontrar mejor valor en Asami comparado con plataformas tradicionales, especialmente al promocionar productos en industrias restringidas o poco atendidas por redes publicitarias Web2. Algunos Anunciantes pueden actuar como intermediarios o agencias, facilitando la creación de campañas para otros, obteniendo un margen y acumulando tokens ASAMI en el proceso.

- **Los Colaboradores** se benefician de una plataforma en expansión con más campañas, mejores oportunidades de pago y mayor visibilidad. También pueden operar cuentas comunitarias de forma colectiva, haciendo crecer su influencia como grupo y dividiendo las recompensas. Asami puede formar parte de múltiples flujos de ingresos vinculados a su presencia online.

Estas posibilidades emprendedoras hacen que **cualquier participante pueda convertirse en una unidad de negocio** dentro de Asami: atrayendo campañas, incorporando Colaboradores o incluso ofreciendo servicios adicionales como creación de contenido o análisis de redes sociales.

### 9.1 Incentivos y el problema del polizón

Como en muchos sistemas descentralizados, Asami enfrenta el desafío clásico de hacer crecer un recurso compartido y abierto: **quienes promueven o financian su crecimiento crean valor que beneficia a todos los participantes**, incluidos aquellos que no contribuyen.

Este “problema del polizón” es una limitación común en las redes abiertas. Sin embargo, Asami introduce una solución elegante y parcial:

- Al comprar y mantener **tokens ASAMI**, cualquier actor que invierte en el crecimiento de la comunidad también adquiere un derecho proporcional sobre los ingresos futuros del protocolo.
- Si sus esfuerzos tienen éxito y se procesan más campañas y colaboraciones, el valor de sus tokens aumenta tanto por apreciación del precio como por reparto de ingresos en DOC.

Esto significa que **promover Asami no es un acto de caridad, sino una inversión**. Los participantes que construyen audiencias, atraen campañas o suman nuevos Colaboradores, a menudo verán un retorno sobre su esfuerzo si también mantienen tokens.

De esta manera, Asami combina la construcción comunitaria altruista con incentivos financieros, creando un ecosistema donde el crecimiento es de interés para todos, sin exigir que todos contribuyan por igual. Esto garantiza flexibilidad, al mismo tiempo que traza un camino hacia la sostenibilidad.

### 9.2 Infraestructura de apoyo

Los esfuerzos de crecimiento de la comunidad también cuentan con el apoyo de aliados externos como **Rootstock Labs** y **Rootstock Collective**, quienes han contribuido con infraestructura, apoyo promocional y financiamiento.

El actual Administrador de campañas está activamente buscando inversión tradicional basada en un modelo de negocio claro, profesionalizando aún más este rol crítico dentro del protocolo. Al mismo tiempo, sigue abierta la posibilidad de que surjan nuevos Administradores de campañas que contribuyan al crecimiento en paralelo o en competencia.

En resumen, el crecimiento de Asami es de base comunitaria, está alineado con incentivos, y es descentralizado por diseño. Cualquiera puede participar, y quienes lo hagan tienen herramientas para capturar el valor que ayudan a crear.

---

## 10. Panorama competitivo

El panorama competitivo de Asami abarca diversas plataformas e iniciativas que integran tecnología blockchain con redes sociales para mejorar el marketing de influencia, la monetización de contenido y la participación comunitaria. A continuación se presenta una visión ampliada de los principales competidores y proyectos relacionados:

### Plataformas descentralizadas de marketing de influencia

Varias plataformas basadas en blockchain han intentado facilitar interacciones directas entre marcas e influencers, buscando eliminar intermediarios y aumentar la transparencia. Algunos ejemplos incluyen:

- **D-creator** (ahora inactiva)
- [**Chirpley**](https://chirpley.ai/vision-and-mission/), centrada en el ecosistema de microinfluencers.

### Redes sociales con funciones de monetización

Las plataformas sociales tradicionales han integrado funciones que permiten a los usuarios monetizar su contenido, compitiendo indirectamente con el modelo de Asami:

- **Facebook e Instagram**: Ofrecen publicaciones y stories comprables, permitiendo a influencers y marcas etiquetar productos directamente en el contenido, facilitando experiencias de comercio electrónico.

- **TikTok**: Conocido por su alto nivel de interacción con audiencias jóvenes, TikTok permite colaboraciones entre influencers y marcas mediante contenido patrocinado y su Creator Marketplace.

### Redes sociales basadas en blockchain

Algunas plataformas combinan funciones sociales con blockchain para ofrecer compartición de contenido y monetización descentralizada:

- **Steemit**: Una red social y plataforma de blogging basada en blockchain donde los usuarios ganan criptomonedas (STEEM) por publicar y curar contenido. Su modelo incentiva la creación y participación mediante recompensas en tokens.

- **BitClout (ahora DeSo)**: Una red social de código abierto basada en blockchain en la que los usuarios pueden comprar y vender “monedas de creador” vinculadas a la reputación de individuos. Busca descentralizar las redes sociales y ofrecer nuevas vías de monetización a creadores de contenido.

### Plataformas que venden republicaciones a Anunciantes

Aunque no siempre se basan en blockchain, existen plataformas especializadas en facilitar la compra de republicaciones y otras formas de interacción social:

- **WeAre8**: Una red social que paga a los usuarios por ver anuncios y participar con contenido. Comparte una porción significativa de los ingresos con los usuarios, promoviendo una distribución más equitativa del valor publicitario.

### Proyectos que miden la influencia en redes sociales

Evaluar la influencia en redes sociales es crucial para campañas de marketing efectivas. Algunos proyectos enfocados en este aspecto son:

- **FeiXiaoHao**: Plataforma de datos sobre blockchain que analiza métricas sociales para ofrecer a los usuarios información sobre el sentimiento del mercado, proyectos en tendencia y posibles movimientos en el ecosistema cripto.

- **TweetBoost**: Estudio que analiza el impacto de las redes sociales en la valoración de NFTs, subrayando el papel determinante de la interacción social en el valor de los activos digitales.

### Tendencias emergentes y consideraciones

La integración de blockchain en redes sociales y marketing de influencia responde al deseo de mayor transparencia, seguridad y empoderamiento del usuario. Las plataformas descentralizadas buscan abordar problemas como la privacidad de los datos, la compensación justa y la autenticidad del contenido. Sin embargo, aún persisten desafíos como la incertidumbre regulatoria, la complejidad técnica y la necesidad de adopción masiva.

Asami se distingue ofreciendo un protocolo de colaboración descentralizado que monetiza la influencia genuina en redes sociales, asegurando una compensación justa e interacciones transparentes entre Anunciantes, Colaboradores y Administradores de campañas. Al aprovechar la tecnología blockchain, Asami apunta a resolver problemas comunes del marketing de influencia como el fraude, la falta de transparencia y la ineficiencia en la gestión de campañas.

---

## 11. Por qué usar Asami

Asami ofrece una nueva forma para que Anunciantes y Colaboradores se conecten en un entorno más auténtico, justo y descentralizado. Las plataformas sociales tradicionales priorizan sus propios ingresos mediante anuncios intrusivos, ofrecen escasa transparencia y a menudo limitan qué industrias pueden publicitar. Asami propone un enfoque diferente: conectar directamente a los Anunciantes con personas dispuestas a amplificar su mensaje —en sus propios términos.

### 11.1 Para Anunciantes

Los Anunciantes eligen Asami porque les permite:

- **Alcanzar personas reales, a través de personas reales**: En lugar de depender de algoritmos de plataforma o anuncios impersonales, Asami permite que tu contenido sea republicado voluntariamente por individuos que creen en lo que ofreces. Esto genera aval genuino y prueba social.

- **Controlar la seguridad de marca**: Con Asami, tú eliges al Administrador de campañas y defines tus propias reglas sobre quién puede participar. Puedes usar listas blancas o negras para asegurarte de que tu contenido llegue solo a audiencias confiables.

- **Anunciar donde otros no pueden**: Muchas industrias, como la cripto o causas políticamente sensibles, enfrentan censura o restricciones en plataformas tradicionales. Asami te brinda un espacio para ejecutar campañas que de otro modo estarían limitadas.

- **Obtener más valor por dólar**: Hacer “boost” de una publicación en X (anteriormente Twitter) puede costar USD 100 para alcanzar entre 24.000 y 57.000 impresiones, pero esas impresiones carecen del respaldo de una voz confiable. Asami genera impresiones mediante voces creíbles, que tienden a convertir mejor y generar valor de marca duradero.

- **Complementar tus estrategias existentes**: Asami no busca reemplazar tu estrategia de marketing actual —es una adición poderosa. Puedes combinar campañas publicitarias tradicionales con campañas en Asami para obtener visibilidad por capas y mayor alcance.

- **Tener transparencia y control de costos**: Tú defines el presupuesto, los rangos de recompensa y la duración de la campaña. Los fondos solo se usan cuando los Colaboradores republican y son registrados por el Administrador de campañas. Los fondos no usados se te devuelven.

### 11.2 Para Colaboradores

Los Colaboradores se unen a Asami porque les permite:

- **Monetizar su influencia social**: Aunque no seas un creador de contenido tradicional, puedes tener verdadera influencia sobre tus seguidores. Asami te permite ganar dinero por amplificar mensajes que se alinean con tus valores.

- **Mantener tu independencia**: Tú eliges qué campañas apoyar. No necesitas comprometer tu voz ni tu reputación —no hay obligación de participar en campañas con las que no estés de acuerdo.

- **Recibir pagos globalmente, en cripto**: Las recompensas se pagan en DOC, una stablecoin indexada al dólar. Los pagos van directo a tu billetera y son utilizables desde cualquier parte del mundo —sin intermediarios ni demoras bancarias.

- **Hacer crecer tu perfil**: Asami recompensa no solo a los grandes influencers, sino también a los microinfluencers genuinos. Si tu audiencia interactúa contigo de forma auténtica, serás reconocido y compensado.

- **Participar en un sistema con propósito**: Cada vez que colaboras en una campaña, ganas tokens ASAMI. Estos tokens te dan una participación en los ingresos del protocolo y voz en su evolución. Dejas de ser solo un usuario: te conviertes en parte interesada.

- **Incorporarte fácilmente**: La plataforma ofrece retiros sin gas y entrada desde Web2, así que incluso si nunca usaste cripto antes, puedes unirte a Asami y recibir tus recompensas sin fricciones.

Asami está pensado para personas —personas con algo para decir, algo que promover y que se preocupan por cómo se propagan los mensajes. Ya sea que estés aquí para anunciar o colaborar, Asami te ofrece una mejor manera de conectar y crecer.

---

## 12. Direccionse de contrato y enlaces { .no-cols }

**Contrato inteligente de Asami** (Rootstock):  
  https://explorer.rootstock.io/address/0x3150e390bc96f1b4a05cf5183b8e7bdb68566954

**Dirección actual del Administrador de campañas**:  
  https://explorer.rootstock.io/address/0x3e79325b61d941e7996f0a1aad4f66a703e24faa

**Información sobre la stablecoin DOC (Money on Chain)**:  
  https://moneyonchain.com/

**Código abierto del software del Administrador de campañas**:  
  https://github.com/constata-eu/asami

---

## 13. Contacto y soporte { .no-cols }

Se alienta a los miembros a hacer preguntas o solicitar ayuda a través de canales públicos. Nuestro objetivo es mantener toda la comunicación transparente y orientada a la comunidad.

**X (Twitter):**
- En inglés: [@asami_club](https://twitter.com/asami_club)
- En español: [@asami_club_es](https://twitter.com/asami_club_es)

**Telegram:**
- Grupo en inglés: `@asami_club`
- Grupo en español: `@asami_club_es`

Ten en cuenta que todas las solicitudes de revisión de puntajes deben seguir el proceso basado en evidencia descrito en este whitepaper, que también contiene un apéndice con preguntas frecuentes.

---

## Apéndice A: Preguntas frecuentes y problemas conocidos { .no-cols }

**P: ¿Por qué no estoy recibiendo campañas?**  
- Es posible que tu puntaje de influencia sea bajo o que falten categorías requeridas. Puedes solicitar una revisión.

**P: ¿Por qué mi puntaje parece demasiado bajo?**  
- Lee la sección de Medición de influencia y sigue el proceso de resolución de disputas explicado en la Sección 3.2.

**P: Antes recibía más pagos que ahora. ¿Por qué?**  
- Puede deberse a presupuestos más bajos por parte de los Anunciantes, cambios en tu puntaje o categoría, o menor actividad en la plataforma / mayor competencia.

**P: Fui bloqueado o excluido por el Administrador de campañas. ¿Puedo seguir participando?**  
- Sí. Asami es una plataforma sin permisos. Puedes colaborar con otro Administrador de campañas o seguir usando tus tokens ASAMI con normalidad.

**Problema conocido:**  
Algunas colaboraciones pueden registrarse después de que una campaña se quede sin fondos. La plataforma las mostrará como “fallidas”, pero podrían compensarse posteriormente si se agregan fondos.
