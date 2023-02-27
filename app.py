from flask import Flask, render_template,request,redirect
import random
import requests
from bs4 import BeautifulSoup
from flask_sqlalchemy import SQLAlchemy
from flask_login import UserMixin

app=Flask(__name__)
app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite:///database.db'
db = SQLAlchemy()
CURRENT_LOBBIES=dict()
def create_database():
    app.app_context().push()
    db.create_all()
create_database()

class Lobbies(db.Model, UserMixin):
    id = db.Column(db.Integer, primary_key=True)
    lobby_number = db.Column(db.String(20), nullable=False)

class Chess:
    @app.route("/",methods=["GET","POST"])
    def main():
        if request.method=="POST":
            lobby_number=str(random.randint(0,400))
            lobby_number_db= Lobbies.query.filter_by(lobby_number=lobby_number).first()
            if lobby_number_db:
                while lobby_number_db:
                    lobby_number=str(random.randint(0,400))
                    lobby_number_db= Lobbies.query.filter_by(lobby_number=lobby_number).first()
            new_lobby=Lobbies(lobby_number=lobby_number)
            db.session.add(new_lobby)
            db.session.commit()
            for lobby in Lobbies.query.all():
                if lobby.lobby_number in CURRENT_LOBBIES.keys():
                    registered_lobby=True
                else:
                    CURRENT_LOBBIES[lobby.lobby_number]=1
            
            redirect(f"/lobby/{lobby_number}")
        return render_template("main.html")
    
    @app.route("/lobby/<string:lobbynumber>")
    def lobby(lobbynumber):
        return render_template("index.html")
    
if __name__=="__main__":
    app.run()
