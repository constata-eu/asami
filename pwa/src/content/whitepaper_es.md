# Whitepaper de Asami Club

Este whitepaper describe el diseño actual y la dirección prevista de Asami.club a mayo de 2026.

---

## 1. Panorama general

Asami.club es el club para comunidades de Rootstock que aprecian a sus miembros.

Su propósito es simple: ayudar a las comunidades a reconocer a las personas que ya les aportan valor. A veces ese valor es divulgación pública. A veces es participación, criterio, confianza o apoyo sostenido. Asami existe para volver ese reconocimiento transparente, modesto y económicamente legible en Rootstock.

El club es una red a la que se accede por invitacion, y se centra en proyectos reales, relaciones existentes, conocimiento comunitario y reputación de largo plazo.

Cada miembro aceptado del club es un **Rootstocker**. Un Rootstocker puede ser:

* una comunidad,
* un representante que actúa por una comunidad,
* un miembro individual de una comunidad,
* o cualquier combinación de esas cosas con el tiempo.

El mismo Rootstocker puede crear campañas, participar en campañas, tener ASAMI, invitar a otras personas al club y ayudar a evaluar a otros miembros.

Asami está construido sobre **Rootstock** y usa **DOC** como moneda de recompensa. El contrato inteligente se mantiene deliberadamente simple: recibe fondos de campaña, permite que los administradores registren colaboraciones y pagos, y emite ASAMI según las reglas del protocolo. Los nuevos comportamientos del producto se implementan fuera de cadena, cambiando cómo se administran las campañas, no cambiando el contrato.

---

## 2. Historia y dirección actual

Asami comenzó como un experimento para reconocer la amplificación online. La intuición original era correcta: las personas que explican proyectos, los recomiendan, los defienden y ayudan a difundirlos realmente crean valor.

Pero las primeras versiones en producción también mostraron un problema. Si la membresía es demasiado abierta y las recompensas se presentan de forma demasiado transaccional, el sistema atrae comportamiento extractivo. La gente optimiza para tareas y pagos, no para alineación, reputación o pertenencia comunitaria.

Por eso Asami cambió de dirección.

La dirección actual del producto es:

* la membresía es principalmente por invitación,
* existe una lista de espera pero no implica aceptación,
* las comunidades deben poder reconocer a miembros que ya conocen,
* las campañas de repost siguen siendo útiles, pero son solo un tipo de campaña,
* y el score se basa en juicio humano calificado, no en métricas automáticas de redes sociales.

En resumen, Asami ya no intenta ser un mercado genérico de influencia social. Está pasando a ser el club de Rootstock para la apreciación comunitaria.

---

## 3. Roles y membresía

### 3.1 Rootstockers

Cada miembro aceptado del club es un **Rootstocker**.

Rootstocker es la identidad general dentro de Asami. Reemplaza cualquier etiqueta separada dentro del club. La idea es reforzar que el club pertenece al ecosistema Rootstock y no fragmentarlo con una identidad social paralela.

La mayoría de los nuevos Rootstockers deberían entrar al club por invitación de:

* una comunidad existente,
* un miembro confiable existente,
* o algún Rootstocker aprobado con capacidad de invitar.

Quienes no tengan invitación todavía pueden postularse a una lista de espera, pero la aceptación es discrecional y no está garantizada.

La aceptación en el club es independiente de la soberanía de la wallet. Una persona puede estar aceptada mientras sigue usando una cuenta administrada, o puede ya controlar su wallet y estar inactiva a nivel del club.

### 3.2 Creadores de campaña

Los creadores de campaña son Rootstockers que financian campañas, normalmente en nombre de una comunidad.

Pueden:

* crear campañas de repost alrededor de un post de X,
* crear campañas de giveaway para su comunidad,
* definir presupuestos,
* elegir el alcance de destinatarios cuando el tipo de campaña lo permite,
* y construir relaciones comunitarias de largo plazo mediante apreciación repetida.

“Creador de campaña” es el término de interfaz preferido donde antes aparecía “anunciante”.

### 3.3 Participantes

Los participantes son Rootstockers que reciben apreciación a través de una campaña.

En campañas de repost, participan haciendo repost o quote tweet del post objetivo en X y manteniéndose elegibles hasta la fase de asignación de recompensas.

En campañas de giveaway, participan por pertenecer al alcance comunitario seleccionado. No hace falta repost.

### 3.4 Administradores de campaña

Los administradores de campaña operan la lógica fuera de cadena alrededor de un contrato inteligente que no cambia.

Ellos:

* determinan elegibilidad,
* reúnen participantes,
* calculan recompensas,
* registran colaboraciones on-chain,
* mantienen datos reputacionales,
* y operan los servicios públicos del frontend y backend.

No necesitan cambiar el contrato inteligente para introducir nuevos comportamientos de campaña. Su poder proviene del software, la discreción, la transparencia y la competencia.

### 3.5 Holders del token

Los holders de ASAMI reciben revenue sharing y votan sobre la tasa de fee del protocolo.

No controlan directamente los resultados de campañas, la admisión al club ni los scores comunitarios. La gobernanza es intencionalmente acotada.

---

## 4. Reputación del club

Asami ya no usa analíticas automáticas de X para calcular el score de los miembros. X ahora se usa solo donde es operacionalmente necesario, principalmente para:

* login y vinculación de cuentas,
* y verificar si un usuario hizo repost o quote tweet del post objetivo.

El score que importa dentro del club es un **score reputacional** construido a partir de evaluaciones humanas registradas dentro de relaciones comunitarias.

### 4.1 Categorías del score

Cada evaluación puede incluir los siguientes atributos, cada uno de **1 a 10**:

* **Alineación**: encaje con los valores de Rootstock y Asami.
* **Confianza**: honestidad, confiabilidad y seguridad para trabajar con esa persona.
* **Contribución**: valor útil que crea para las comunidades.
* **Advocacy**: efectividad cuando ayuda activamente a difundir un mensaje.
* **Criterio**: calidad del juicio sobre qué merece apoyo.

Cada evaluación también puede incluir una **fecha de ingreso a Rootstock** declarada.

El club deriva una **puntuación de antigüedad en Rootstock** a partir de la fecha creíble más temprana presentada para un miembro. Esa fecha se mapea a un valor de 1 a 10 sobre una línea de tiempo móvil entre el inicio de Rootstock y el presente.

### 4.2 Relaciones comunitarias

Asami usa la relación `community_members` como el registro principal que une tres ideas:

* membresía comunitaria,
* score actual de un Rootstocker sobre otro,
* e historial comunitario como colaboraciones y recompensas.

Solo existe una relación mutable por cada par evaluador-objetivo. Re-evaluar actualiza esa misma relación en lugar de crear un historial infinito de registros de puntuación.

Esto implica que:

* si puntúas a alguien, pasa a formar parte de tu vista comunitaria,
* si agregas manualmente a alguien a tu comunidad, la relación puede existir antes de que lo puntúes,
* y esa misma fila luego puede acumular conteos de colaboraciones, recompensas y nuevos scores.

### 4.3 Entradas prioritarias y valores por defecto

Algunas filas de `community_members` pueden marcarse como **priority**.

La fila prioritaria más reciente para un objetivo actúa como la entrada de prioridad general del club para esa persona. No se guarda en una tabla separada solo para admin.

Cualquier campo de score puede ser nulo. Un campo nulo significa “no provisto”, no “bajo”.

Si un creador de campaña tiene un score local sobre una persona pero deja un atributo nulo, ese atributo cae al valor global del club para ese miembro.

Si todavía no se puede construir un valor global para un atributo, Asami usa por defecto:

* `5` para atributos numéricos,
* y `2022-01-01` para la fecha de ingreso a Rootstock.

### 4.4 Cuándo se puede puntuar a alguien

Un Rootstocker no puede puntuar al mismo objetivo más de **una vez cada 30 días**.

Si ese cooldown se cumple, el score solo se habilita bajo reglas explícitas del club:

* **Invitación**: crear una invitación requiere puntuar.
* **Selecciones semanales**: cualquiera puede puntuar a los miembros elegidos esa semana, u omitirlos explícitamente.
* **Relación de campaña**: un creador de campaña solo puede puntuar a un participante después de haberle pagado en total más de **1 DOC**.

Cuando un evento válido habilita el score, abre una **ventana de 15 días** para ese par evaluador-objetivo.

Si alguien no conoce lo suficiente al objetivo, puede crear un registro de omisión en lugar de puntuarlo. Las omisiones registran la relación y el motivo, pero no contienen valores numéricos.

### 4.5 Score base y score del creador

Cada miembro tiene un **score base** y puede además tener un **score específico respecto de un creador**.

El score base se construye a partir de todas las filas de `community_members` donde el usuario aparece como `member_id`:

* 50% desde la entrada prioritaria más reciente, si existe,
* 50% desde el promedio de evaluaciones comunitarias regulares,
* o el lado que exista si solo hay uno.

Para un creador de campaña evaluando a un participante:

* 50% proviene del score base del participante,
* 50% proviene de la evaluación actual de ese creador sobre el participante.

Si el creador no completó algún campo, ese campo cae al valor base.

Esto crea una capa reputacional compartida para el club sin quitarle a cada comunidad el derecho de aplicar su propio juicio local.

### 4.6 Interpretación

El score de Asami no es una verdad universal ni una estimación automática del valor social de una persona.

Es una señal reputacional estructurada para uso práctico dentro de comunidades de Rootstock.

Su propósito es ayudar a las comunidades a decidir:

* a quién invitar,
* a quién incluir en campañas,
* cómo repartir presupuestos de apreciación,
* y cómo reconocer a quienes siguen apareciendo y aportando.

---

## 5. Tipos de campaña

Asami trabaja actualmente con dos tipos principales de campaña.

### 5.1 Campañas de repost en X

Una campaña de repost en X se crea a partir de:

* un presupuesto en DOC,
* una URL de un post de X,
* y una recompensa máxima por participante.

La duración es fija: **20 días**.

Los miembros no ven estas campañas durante toda su vida contractual. En el frontend oficial solo se muestran durante la ventana pensada para participación pública y desaparecen de los flujos de miembros durante la fase final de liquidación.

Operativamente funcionan así:

1. Los miembros expresan intención haciendo repost o quote tweet del post objetivo.
2. El backend verifica esa participación usando el contexto de la propia cuenta de X vinculada del participante.
3. Cinco días antes del vencimiento, el administrador reúne la lista final de participantes elegibles.
4. Las recompensas se prorratean sobre esa lista según el score, sujetas al máximo por participante definido por el creador.
5. Luego las colaboraciones se registran on-chain de forma asíncrona mediante el worker de fondo.

Después de que se reúnen los participantes y se asignan las recompensas, la campaña ya terminó en la práctica para nuevos participantes aunque la fecha final del contrato aún no haya llegado.

Cualquier fondo que quede sin usar permanece en el contrato y se devuelve al creador según el comportamiento normal del protocolo al llegar el vencimiento.

Las campañas de repost no se pueden extender ni volver a fondear desde el frontend oficial.

### 5.2 Campañas de giveaway comunitario

Una campaña de giveaway comunitario no depende de reposts.

En cambio, el creador selecciona:

* a todos los miembros de su comunidad,
* o a un subconjunto específico de esa comunidad.

El backend reúne inmediatamente la lista de destinatarios desde esas relaciones comunitarias y luego calcula las recompensas usando la misma lógica proporcional utilizada por las campañas de repost.

La diferencia principal entre repost y giveaway es cómo se construye la lista de participantes. Una vez que existe la lista final, la asignación de recompensas es la misma:

* ponderada por score,
* limitada por el presupuesto,
* con tope cuando aplique,
* y luego registrada on-chain de forma asíncrona.

### 5.3 Flujo de estados

Operativamente, las campañas pasan por estas etapas fuera de cadena:

* **published**: la campaña existe y todavía puede reunir participantes,
* **collabs gathered**: la lista final de participantes ya está cerrada,
* **rewards assigned**: cada colaboración ya tiene un monto final,
* y recién después el worker puede enviarlas on-chain.

Solo las campañas con recompensas asignadas son elegibles para el job on-chain de colaboraciones.

---

## 6. Tokenomics e incentivos

El **token ASAMI** es el activo nativo del protocolo Asami.club.

Tiene un suministro máximo fijo de **21 millones de tokens** y se distribuye a partir del uso real del protocolo, no por preventa ni premine.

### 6.1 Fee del protocolo

Cada vez que el protocolo paga recompensas en DOC, retiene una parte como fee.

Ese fee financia:

* la emisión de ASAMI,
* el revenue sharing,
* y la alineación económica del club.

### 6.2 Revenue sharing

Cada ciclo de 15 días, los holders elegibles de ASAMI reciben una parte del fee pool del protocolo en DOC.

Para calificar, un holder debe:

* mantener el umbral requerido de ASAMI,
* no mover esos tokens durante el ciclo,
* y tenerlos reclamados en una wallet elegible.

Los umbrales y ventanas exactas se ven en la app en vivo y en el estado del contrato.

### 6.3 Split de emisión

La emisión de ASAMI sigue la actividad real del protocolo. Cuando se pagan colaboraciones, ASAMI se distribuye entre los tres roles operativos:

* el **administrador de campaña**,
* el **participante**,
* y el **creador de campaña**.

El split vigente sigue siendo el que ya está codificado en el contrato inteligente.

### 6.4 Gobernanza

La gobernanza de ASAMI es intencionalmente acotada.

Hoy los holders solo votan sobre la **tasa de fee del protocolo**. No votan directamente sobre:

* quién entra al club,
* cómo se administra una campaña en particular,
* ni cómo se puntúa a un miembro específico.

---

## 7. Arquitectura técnica

### 7.1 Por qué Rootstock

Asami existe específicamente para el ecosistema Rootstock.

Rootstock fue elegido porque ofrece:

* seguridad alineada con Bitcoin,
* compatibilidad EVM,
* operación estable,
* y un ecosistema nativo donde DOC ya tiene sentido como moneda de recompensa comunitaria.

### 7.2 DOC como moneda de recompensa

Los presupuestos y pagos de campaña se denominan en **DOC (Dollar on Chain)**.

DOC se usa porque es:

* estable,
* nativo de Rootstock,
* y más práctico que activos volátiles para reconocer contribuciones comunitarias.

### 7.3 El contrato inteligente y la capa off-chain

El contrato inteligente actual es deliberadamente acotado. Se encarga de:

* recibir fondos de campaña,
* registrar colaboraciones,
* pagar recompensas,
* y emitir ASAMI según las reglas del protocolo.

El contrato **no** decide:

* quién debería ser invitado,
* quién debería ser puntuado,
* cómo se construye la reputación comunitaria,
* qué reposts cuentan,
* ni cómo se eligen destinatarios de giveaways.

Esas decisiones viven en el software off-chain del administrador de campañas.

### 7.4 Software open source

El stack de Asami es open source y está construido principalmente con **Rust** en el backend y un frontend web para operaciones de miembros y administración.

Ese software maneja:

* gestión de cuentas,
* administración de campañas,
* relaciones comunitarias,
* score,
* vinculación de cuentas de X,
* verificación de reposts,
* y flujos de onboarding sin gas.

### 7.5 Reclamos sin gas y onboarding

Asami soporta flujos de reclamo sin gas para ayudar a personas nuevas en Rootstock a recibir recompensas y pasar gradualmente a una wallet autosoberana.

El modelo de cuentas del club separa:

* soberanía de wallet,
* y aceptación dentro del club.

Eso permite incorporar personas antes de que se sientan completamente cómodas con wallets de Rootstock, manteniendo al mismo tiempo un camino hacia el control total.

---

## 8. Gobernanza, confianza y prevención de abuso

Asami no está gobernado por una empresa ni por una DAO amplia que resuelva cada disputa.

Está gobernado por:

* el contrato inteligente,
* el software open source,
* las reglas de curaduría del club,
* y la discreción práctica de administradores de campaña y operadores comunitarios.

### 8.1 Sin garantías de servicio

Participar en Asami es voluntario y se ofrece tal como está.

No existe garantía de que:

* una campaña incluya a un participante específico,
* una persona en lista de espera sea aceptada,
* un creador de campaña siga financiando campañas,
* ni que un administrador opere siempre con una sola filosofía.

### 8.2 Prevención de abuso

La superficie principal de abuso hoy es distinta de la de las primeras versiones.

Los riesgos principales ahora son:

* farming extractivo de membresía,
* reposts de bajo esfuerzo sin alineación comunitaria,
* manipulación de scores relacionales,
* abuso de invitaciones,
* y creación de comunidades spam solo para abrir permisos de score.

El club mitiga estos riesgos mediante:

* curaduría de invitaciones,
* discreción sobre la lista de espera,
* ventanas de score,
* cooldowns,
* el umbral de `> 1 DOC` antes de habilitar score basado en campaña,
* relaciones reputacionales públicas,
* y supervisión administrativa.

### 8.3 Transparencia

Las acciones financieras son públicas on-chain.

Las relaciones comunitarias, scores y participaciones son más sociales que contractuales, pero aun así están pensadas para ser lo suficientemente visibles como para permitir escrutinio, revisión y comparación.

### 8.4 Competencia

Los administradores de campaña no tienen estatus especial dentro del protocolo.

Cualquier persona capaz de operar software compatible puede construir su propio frontend, curar sus propias comunidades y administrar campañas de otro modo.

Esa posibilidad es una parte importante del modelo de gobernanza de Asami.

---

## 9. Encaje de mercado

Asami no intenta reemplazar quests, redes de anuncios ni mercados masivos de promoción social.

Sirve para algo más específico:

* ayudar a comunidades de Rootstock a apreciar a miembros conocidos,
* ayudarlas a descubrir gradualmente a nuevos miembros alineados,
* y dar recompensas públicas, modestas y nativas de Rootstock a participaciones significativas.

Por eso Asami es complementario a:

* herramientas de onboarding,
* sistemas de quests,
* programas comunitarios,
* y canales tradicionales de marketing.

Su rasgo distintivo no es el alcance masivo. Es la **confianza moldeada por comunidad**.

---

## 10. Dirección futura

Las próximas etapas de Asami están centradas en fortalecer el modelo de club, no en expandirse hacia un mercado genérico de atención.

Las líneas probables de desarrollo incluyen:

* mejores herramientas de revisión para lista de espera e invitaciones,
* campañas comunitarias más ricas,
* mejores flujos de score para miembros,
* mejores vistas e historial comunitario,
* flujos de funding con BTC y Lightning que sigan liquidando bajo lógica DOC,
* y herramientas más refinadas para operaciones comunitarias de Rootstock.

Asami no está enfocado hoy en convertirse en un mercado multiplataforma de atención. Su hogar estratégico es Rootstock.

---

## 11. Consideraciones legales y de datos

Asami es un protocolo y un stack de software, no una promesa de servicio, retorno de inversión ni respaldo institucional.

### 11.1 No es una oferta de inversión

ASAMI no se presenta como un valor negociable ni como una promesa de retorno garantizado.

Es un activo nativo del protocolo vinculado a participación, fee sharing y gobernanza.

### 11.2 Sin garantía de admisión ni de recompensa

La admisión al club es discrecional.

La participación en campañas es discrecional.

Las recompensas dependen de las reglas de campaña, del comportamiento del administrador, de la liquidación on-chain y de los límites del protocolo.

### 11.3 Responsabilidades del usuario

Cada miembro es responsable de:

* cumplir con sus leyes locales,
* hacer cualquier divulgación de promoción paga que corresponda,
* y entender las implicancias fiscales de recibir recompensas.

### 11.4 Uso de datos

Asami usa:

* datos on-chain,
* datos de cuentas y relaciones comunitarias,
* y datos limitados de X necesarios para vincular cuentas y verificar reposts.

No depende de analíticas premium globales de X para construir scores.

Si tienes inquietudes sobre datos o moderación, contacta a la comunidad públicamente a través de:

* [@asami_club](https://twitter.com/asami_club)
* [@asami_club_es](https://twitter.com/asami_club_es)
* [@asami_club](https://t.me/asami_club)
* [@asami_club_es](https://t.me/asami_club_es)

---

## 12. Notas finales

Asami.club es un experimento nativo de Rootstock en apreciación comunitaria.

El protocolo es público. Los contratos inteligentes son permissionless. El club, en cambio, es curado.

Si eres Rootstocker, participa en una campaña cuando tenga sentido.

Si eres una comunidad, crea una.

Si eres desarrollador, construye sobre el protocolo.

La forma final de Asami dependerá de la calidad de las comunidades que decidan usarlo.

---

## Apéndice A: Direcciones de contrato y enlaces { .no-cols }

**Contrato inteligente de Asami** (Rootstock):  
https://explorer.rootstock.io/address/0x3150e390bc96f1b4a05cf5183b8e7bdb68566954

**Dirección del administrador de campaña actual**:  
https://explorer.rootstock.io/address/0x3e79325b61d941e7996f0a1aad4f66a703e24faa

**Información de DOC (Money on Chain)**:  
https://moneyonchain.com/

**Software open source de Asami**:  
https://github.com/constata-eu/asami

---

## Apéndice B: Contacto y soporte { .no-cols }

Se alienta a los miembros a hacer preguntas por canales públicos.

**X:**

* Inglés: [@asami_club](https://twitter.com/asami_club)
* Español: [@asami_club_es](https://twitter.com/asami_club_es)

**Telegram:**

* Grupo en inglés: [@asami_club](https://t.me/asami_club)
* Grupo en español: [@asami_club_es](https://t.me/asami_club_es)

---

## Apéndice C: FAQ { .no-cols }

**P: ¿Por qué no veo campañas?**

* Puede que aún no seas un miembro activo del club, que tus relaciones comunitarias no te vuelvan elegible o que simplemente no haya campañas abiertas para ti en este momento.

**P: ¿Por qué mi score parece bajo?**

* Los scores ahora se construyen a partir de evaluaciones comunitarias y entradas prioritarias, no de métricas automáticas de X. Revisa la sección de reputación y pide revisión si una relación o score fue actualizado de forma incorrecta.

**P: ¿Por qué recibí menos que otro participante?**

* Las recompensas dependen del presupuesto de campaña, la lista final de participantes, el score y cualquier tope por usuario que haya definido el creador.

**P: Me dejaron en la lista de espera o me marcaron como inactivo. ¿Igual puedo usar Asami?**

* No necesariamente dentro del contexto del club por defecto. La aceptación del club es curada.
