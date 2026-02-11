<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Zadajte prihlasovacie meno vo formte EX_cc2d76</name>
   <tag></tag>
   <elementGuidId>3127270b-a3b3-429a-bec9-232abe040c41</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Zadajte prihlasovacie meno vo formáte EXXXXXXXX (X sú číslice).'])[1]/following::div[1]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value></value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>b44691d8-f59c-456e-807f-1cd1cc6aceca</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>govuk-form-group</value>
      <webElementGuid>efa0b786-3a07-4d15-990b-1c41ff550756</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                Heslo *
                

    /* Skry defaultné password reveal tlačidlá */
    input[type=&quot;password&quot;]::-ms-reveal,
    input[type=&quot;password&quot;]::-ms-clear {
        display: none;
    }
    
    input[type=&quot;password&quot;]::-webkit-textfield-decoration-container {
        visibility: hidden;
        pointer-events: none;
        position: absolute;
    }

    /* Potlačenie štýlov z .login-control input pre checkbox */
    .login-control input[type=&quot;checkbox&quot;] {
        display: inline-block !important;
        width: auto !important;
        height: auto !important;
        color: inherit !important;
        padding: 0 !important;
        border: 1px solid #999 !important;
        border-radius: 3px !important;
        background: #fff !important;
        box-sizing: border-box !important;
        vertical-align: middle !important;
    }

    /* Oprava zarovnania labelu s checkboxom keď nie je IDSK CSS dostupné */
    .login-control .govuk-checkboxes__label {
        vertical-align: middle !important;
        line-height: 1 !important;
        margin: 0 !important;
        display: inline-block !important;
        position: relative !important;
        top: 2px !important;
    }


                
                    
                        
                        
                            Zobraziť heslo
                        
                    
                


(function() {
    'use strict';
    
    const checkbox = document.getElementById('chk_pwd_8035d1ee');
    const passwordInput = document.querySelector('[data-pwd-toggle-id=&quot;pwd_8035d1ee&quot;]');
    
    if (!checkbox || !passwordInput) return;
    
    // Event handler pre checkbox
    checkbox.addEventListener('change', function() {
        passwordInput.type = this.checked ? 'text' : 'password';
    });
    
    // MutationObserver pre obojstrannú synchronizáciu (ak je podporovaný)
    if (typeof MutationObserver !== 'undefined') {
        const observer = new MutationObserver(function(mutations) {
            mutations.forEach(function(mutation) {
                if (mutation.type === 'attributes' &amp;&amp; mutation.attributeName === 'type') {
                    const isText = (passwordInput.type === 'text');
                    if (checkbox.checked !== isText) {
                        checkbox.checked = isText;
                    }
                }
            });
        });
        
        observer.observe(passwordInput, {
            attributes: true,
            attributeFilter: ['type']
        });
    }
})();


            </value>
      <webElementGuid>75d24330-2c3b-4678-ae7b-fc8704f06ac3</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]/div[@class=&quot;wrapper-full&quot;]/div[@class=&quot;govuk-width-container&quot;]/div[@class=&quot;govuk-main-wrapper&quot;]/form[1]/div[@class=&quot;govuk-grid-row&quot;]/div[@class=&quot;govuk-grid-column-one-half&quot;]/div[@class=&quot;govuk-form-group&quot;]</value>
      <webElementGuid>a05399bd-da8f-4645-b066-dda60a8911a4</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Zadajte prihlasovacie meno vo formáte EXXXXXXXX (X sú číslice).'])[1]/following::div[1]</value>
      <webElementGuid>814db2f4-4220-4790-87d4-0b2abb0ccd36</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='*'])[2]/following::div[1]</value>
      <webElementGuid>4bac8d03-a1aa-406c-b89b-9655b77b0766</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//form/div/div/div[2]</value>
      <webElementGuid>718efe75-4607-4d53-881d-171636e63092</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
                Heslo *
                

    /* Skry defaultné password reveal tlačidlá */
    input[type=&quot;password&quot;]::-ms-reveal,
    input[type=&quot;password&quot;]::-ms-clear {
        display: none;
    }
    
    input[type=&quot;password&quot;]::-webkit-textfield-decoration-container {
        visibility: hidden;
        pointer-events: none;
        position: absolute;
    }

    /* Potlačenie štýlov z .login-control input pre checkbox */
    .login-control input[type=&quot;checkbox&quot;] {
        display: inline-block !important;
        width: auto !important;
        height: auto !important;
        color: inherit !important;
        padding: 0 !important;
        border: 1px solid #999 !important;
        border-radius: 3px !important;
        background: #fff !important;
        box-sizing: border-box !important;
        vertical-align: middle !important;
    }

    /* Oprava zarovnania labelu s checkboxom keď nie je IDSK CSS dostupné */
    .login-control .govuk-checkboxes__label {
        vertical-align: middle !important;
        line-height: 1 !important;
        margin: 0 !important;
        display: inline-block !important;
        position: relative !important;
        top: 2px !important;
    }


                
                    
                        
                        
                            Zobraziť heslo
                        
                    
                


(function() {
    &quot; , &quot;'&quot; , &quot;use strict&quot; , &quot;'&quot; , &quot;;
    
    const checkbox = document.getElementById(&quot; , &quot;'&quot; , &quot;chk_pwd_8035d1ee&quot; , &quot;'&quot; , &quot;);
    const passwordInput = document.querySelector(&quot; , &quot;'&quot; , &quot;[data-pwd-toggle-id=&quot;pwd_8035d1ee&quot;]&quot; , &quot;'&quot; , &quot;);
    
    if (!checkbox || !passwordInput) return;
    
    // Event handler pre checkbox
    checkbox.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
        passwordInput.type = this.checked ? &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;;
    });
    
    // MutationObserver pre obojstrannú synchronizáciu (ak je podporovaný)
    if (typeof MutationObserver !== &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
        const observer = new MutationObserver(function(mutations) {
            mutations.forEach(function(mutation) {
                if (mutation.type === &quot; , &quot;'&quot; , &quot;attributes&quot; , &quot;'&quot; , &quot; &amp;&amp; mutation.attributeName === &quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;) {
                    const isText = (passwordInput.type === &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;);
                    if (checkbox.checked !== isText) {
                        checkbox.checked = isText;
                    }
                }
            });
        });
        
        observer.observe(passwordInput, {
            attributes: true,
            attributeFilter: [&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;]
        });
    }
})();


            &quot;) or . = concat(&quot;
                Heslo *
                

    /* Skry defaultné password reveal tlačidlá */
    input[type=&quot;password&quot;]::-ms-reveal,
    input[type=&quot;password&quot;]::-ms-clear {
        display: none;
    }
    
    input[type=&quot;password&quot;]::-webkit-textfield-decoration-container {
        visibility: hidden;
        pointer-events: none;
        position: absolute;
    }

    /* Potlačenie štýlov z .login-control input pre checkbox */
    .login-control input[type=&quot;checkbox&quot;] {
        display: inline-block !important;
        width: auto !important;
        height: auto !important;
        color: inherit !important;
        padding: 0 !important;
        border: 1px solid #999 !important;
        border-radius: 3px !important;
        background: #fff !important;
        box-sizing: border-box !important;
        vertical-align: middle !important;
    }

    /* Oprava zarovnania labelu s checkboxom keď nie je IDSK CSS dostupné */
    .login-control .govuk-checkboxes__label {
        vertical-align: middle !important;
        line-height: 1 !important;
        margin: 0 !important;
        display: inline-block !important;
        position: relative !important;
        top: 2px !important;
    }


                
                    
                        
                        
                            Zobraziť heslo
                        
                    
                


(function() {
    &quot; , &quot;'&quot; , &quot;use strict&quot; , &quot;'&quot; , &quot;;
    
    const checkbox = document.getElementById(&quot; , &quot;'&quot; , &quot;chk_pwd_8035d1ee&quot; , &quot;'&quot; , &quot;);
    const passwordInput = document.querySelector(&quot; , &quot;'&quot; , &quot;[data-pwd-toggle-id=&quot;pwd_8035d1ee&quot;]&quot; , &quot;'&quot; , &quot;);
    
    if (!checkbox || !passwordInput) return;
    
    // Event handler pre checkbox
    checkbox.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
        passwordInput.type = this.checked ? &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;;
    });
    
    // MutationObserver pre obojstrannú synchronizáciu (ak je podporovaný)
    if (typeof MutationObserver !== &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
        const observer = new MutationObserver(function(mutations) {
            mutations.forEach(function(mutation) {
                if (mutation.type === &quot; , &quot;'&quot; , &quot;attributes&quot; , &quot;'&quot; , &quot; &amp;&amp; mutation.attributeName === &quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;) {
                    const isText = (passwordInput.type === &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;);
                    if (checkbox.checked !== isText) {
                        checkbox.checked = isText;
                    }
                }
            });
        });
        
        observer.observe(passwordInput, {
            attributes: true,
            attributeFilter: [&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;]
        });
    }
})();


            &quot;))]</value>
      <webElementGuid>999a57d2-9b53-41dd-be10-86ffed4fc7af</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
