# Connect UFO Cut to an AI assistant

[English](#english) · [Español](#español) · [العربية](#العربية)

---

## English

UFO Cut can be driven by an AI assistant. Once connected, you can ask it in
plain words to make a project, import clips, cut, add titles and captions,
and export. It uses MCP (Model Context Protocol), which Claude and many
other assistants speak.

### The fast way

1. Open UFO Cut and go to **Settings › Automation**.
2. Under **AI assistants**, press one of the two copy buttons. Your own
   program path is already filled in.
3. Paste it where the steps below say, then restart the assistant.

### Where the program is

The examples use this path. If you installed UFO Cut "for everyone", it is
`C:\Program Files\UFO Cut\ufocut.exe` instead.

```text
C:\Users\YOUR-NAME\AppData\Local\Programs\UFO Cut\ufocut.exe
```

### Claude Code

Run this once in a terminal (PowerShell):

```powershell
claude mcp add ufo-cut -- "$env:LOCALAPPDATA\Programs\UFO Cut\ufocut.exe" --mcp
```

### Claude desktop app

Open **Settings › Developer › Edit Config** and add:

```json
{
  "mcpServers": {
    "ufo-cut": {
      "command": "C:\\Users\\YOUR-NAME\\AppData\\Local\\Programs\\UFO Cut\\ufocut.exe",
      "args": ["--mcp"]
    }
  }
}
```

Save, then quit and reopen Claude.

### Other assistants

Any app that can start a local MCP server takes the same two values:

| Setting | Value |
| --- | --- |
| Command | the full path to `ufocut.exe` |
| Arguments | `--mcp` |

Apps that read an `mcpServers` block (Cursor, Windsurf, VS Code and others)
take the same JSON as the Claude desktop app. OpenAI's Codex command line
takes it in `config.toml`:

```toml
[mcp_servers.ufo-cut]
command = 'C:\Users\YOUR-NAME\AppData\Local\Programs\UFO Cut\ufocut.exe'
args = ["--mcp"]
```

**ChatGPT:** at the time of writing, the ChatGPT app connects to MCP servers
that run on the internet, not to a program on your own PC, so it cannot
start UFO Cut directly. Use one of the apps above, or check ChatGPT's
current help pages, since this changes often.

### Working with the window open

By default the assistant runs UFO Cut's engine in the background, with no
window. To have it work through the window you have open, turn on
**Settings › Automation › Scripts and apps** in UFO Cut first. The assistant
edits projects of its own; it does not take over the project you are
editing.

### Try it

Ask the assistant:

> Create a UFO Cut project called "Test", import the video at
> C:\Users\YOUR-NAME\Videos\clip.mp4, trim it to the first 10 seconds, add
> the title "Hola" and export it as MP4.

### If it does not connect

- Check the path: paste it into File Explorer's address bar. It should find
  the file.
- Restart the assistant fully after changing its settings.
- In a JSON file every backslash in the path is written twice (`\\`).
- The assistant can read and write files only under your user folder.

---

## Español

UFO Cut se puede manejar con un asistente de IA. Una vez conectado, le
pides con tus palabras que cree un proyecto, importe clips, corte, agregue
títulos y subtítulos, y exporte. Usa MCP (Model Context Protocol), que
entienden Claude y muchos otros asistentes.

### La forma rápida

1. Abre UFO Cut y ve a **Ajustes › Automatización**.
2. En **Asistentes de IA**, pulsa uno de los dos botones de copiar. La ruta
   de tu programa ya viene incluida.
3. Pégalo donde indican los pasos de abajo y reinicia el asistente.

### Dónde está el programa

Los ejemplos usan esta ruta. Si instalaste UFO Cut "para todos", está en
`C:\Program Files\UFO Cut\ufocut.exe`.

```text
C:\Users\TU-NOMBRE\AppData\Local\Programs\UFO Cut\ufocut.exe
```

### Claude Code

Ejecuta esto una vez en una terminal (PowerShell):

```powershell
claude mcp add ufo-cut -- "$env:LOCALAPPDATA\Programs\UFO Cut\ufocut.exe" --mcp
```

### App de escritorio de Claude

Abre **Settings › Developer › Edit Config** y agrega:

```json
{
  "mcpServers": {
    "ufo-cut": {
      "command": "C:\\Users\\TU-NOMBRE\\AppData\\Local\\Programs\\UFO Cut\\ufocut.exe",
      "args": ["--mcp"]
    }
  }
}
```

Guarda, cierra Claude por completo y vuelve a abrirlo.

### Otros asistentes

Cualquier app que pueda iniciar un servidor MCP local usa los mismos dos
valores: el **comando** es la ruta completa a `ufocut.exe` y el
**argumento** es `--mcp`. Las apps que leen un bloque `mcpServers` (Cursor,
Windsurf, VS Code y otras) aceptan el mismo JSON de arriba.

**ChatGPT:** al momento de escribir esto, la app de ChatGPT se conecta a
servidores MCP en internet y no a un programa en tu PC, así que no puede
iniciar UFO Cut directamente. Usa una de las apps anteriores o revisa la
ayuda actual de ChatGPT, porque esto cambia seguido.

### Con la ventana abierta

Por defecto el asistente usa el motor de UFO Cut en segundo plano, sin
ventana. Para que trabaje a través de la ventana abierta, activa antes
**Ajustes › Automatización › Scripts y apps** en UFO Cut. El asistente
edita proyectos propios; no toma el proyecto que estás editando.

### Pruébalo

Pídele al asistente:

> Crea un proyecto de UFO Cut llamado "Prueba", importa el video
> C:\Users\TU-NOMBRE\Videos\clip.mp4, recórtalo a los primeros 10 segundos,
> agrega el título "Hola" y expórtalo en MP4.

### Si no conecta

- Revisa la ruta: pégala en la barra de direcciones del Explorador de
  archivos. Debe encontrar el archivo.
- Reinicia el asistente por completo después de cambiar sus ajustes.
- En un archivo JSON cada barra invertida de la ruta se escribe dos veces
  (`\\`).
- El asistente solo puede leer y escribir archivos dentro de tu carpeta de
  usuario.

---

## العربية

يمكن تشغيل UFO Cut بواسطة مساعد ذكاء اصطناعي. بعد الربط تطلب منه بكلماتك
أن ينشئ مشروعًا ويستورد المقاطع ويقص ويضيف العناوين والترجمة ويصدّر. يعتمد
ذلك على بروتوكول MCP الذي يفهمه Claude ومساعدون كثيرون غيره.

### الطريقة السريعة

1. افتح UFO Cut واذهب إلى **الإعدادات › الأتمتة**.
2. تحت **مساعدو الذكاء الاصطناعي** اضغط أحد زرّي النسخ. مسار برنامجك موجود
   فيه مسبقًا.
3. الصقه في المكان المذكور أدناه ثم أعد تشغيل المساعد.

### مكان البرنامج

الأمثلة تستخدم هذا المسار. إذا ثبّتَّ UFO Cut «للجميع» فالمسار هو
`C:\Program Files\UFO Cut\ufocut.exe`.

```text
C:\Users\YOUR-NAME\AppData\Local\Programs\UFO Cut\ufocut.exe
```

### Claude Code

نفّذ هذا السطر مرة واحدة في الطرفية (PowerShell):

```powershell
claude mcp add ufo-cut -- "$env:LOCALAPPDATA\Programs\UFO Cut\ufocut.exe" --mcp
```

### تطبيق Claude لسطح المكتب

افتح **Settings › Developer › Edit Config** وأضف:

```json
{
  "mcpServers": {
    "ufo-cut": {
      "command": "C:\\Users\\YOUR-NAME\\AppData\\Local\\Programs\\UFO Cut\\ufocut.exe",
      "args": ["--mcp"]
    }
  }
}
```

احفظ، ثم أغلق Claude تمامًا وافتحه من جديد.

### مساعدون آخرون

أي تطبيق يستطيع تشغيل خادم MCP محلي يحتاج القيمتين نفسيهما: **الأمر** هو
المسار الكامل إلى `ufocut.exe` و**المعطى** هو `--mcp`. التطبيقات التي تقرأ
قسم `mcpServers` (مثل Cursor و Windsurf و VS Code) تقبل نص JSON نفسه.

**ChatGPT:** حتى وقت كتابة هذا الدليل يتصل تطبيق ChatGPT بخوادم MCP على
الإنترنت وليس ببرنامج على جهازك، لذلك لا يستطيع تشغيل UFO Cut مباشرة.
استخدم أحد التطبيقات السابقة أو راجع صفحات المساعدة الحالية لـ ChatGPT،
فهذا يتغير كثيرًا.

### مع النافذة المفتوحة

افتراضيًا يشغّل المساعد محرّك UFO Cut في الخلفية دون نافذة. ليعمل عبر
النافذة المفتوحة فعّل أولًا **الإعدادات › الأتمتة › السكربتات والتطبيقات**
في UFO Cut. المساعد يحرّر مشاريع خاصة به ولا يستولي على المشروع الذي تعمل
عليه.

### جرّبه

اطلب من المساعد:

> أنشئ مشروع UFO Cut باسم "تجربة"، واستورد الفيديو
> C:\Users\YOUR-NAME\Videos\clip.mp4، واقصصه إلى أول 10 ثوانٍ، وأضف العنوان
> "مرحبا" وصدّره بصيغة MP4.

### إذا لم يتصل

- تحقق من المسار: الصقه في شريط العنوان في مستكشف الملفات. يجب أن يجد
  الملف.
- أعد تشغيل المساعد تمامًا بعد تغيير إعداداته.
- في ملف JSON تُكتب كل شرطة مائلة عكسية في المسار مرتين (`\\`).
- المساعد يقرأ ويكتب الملفات داخل مجلد المستخدم فقط.
